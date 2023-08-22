use std::thread;
use chrono::Utc;
use rocket::tokio::spawn;
use tokio_schedule::{every, Job};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use crate::entities::blacklist;
use crate::entities::prelude::Blacklist;

fn get_nodes() -> Vec<String> {
    return match ureq::get("https://check.torproject.org/torbulkexitlist")
        .call()
    {
        Ok(response) => {
            let mut ips: Vec<String> = Vec::new();
            let text = response.into_string().unwrap();
            let lines = text.lines();

            for line in lines {
                if line.starts_with("#") {
                    continue;
                }
                ips.push(line.to_string());
            }
            info!("Downloaded {} TOR IPs", ips.len());
            ips
        }
        Err(e) => {
            println!("Error downloading TOR IPs: {}", e);
            Vec::new()
        }
    }
}

pub async fn get(db: &DatabaseConnection) {
    let nodes = get_nodes();

    let known_ips: Vec<String> = match Blacklist::find()
        .filter(blacklist::Column::Reason.contains("TOR Node"))
        .all(db)
        .await
    {
        Ok(bans) => bans.into_iter().map(|ban| ban.ip).collect(),
        Err(_) => Vec::new()
    };

    let checks = nodes.iter().filter(|node| {
        !known_ips.contains(node)
    }).collect::<Vec<_>>();

    if checks.len() == 0 {
        info!("No new TOR nodes to add to the blacklist");
        return;
    }

    let now = Utc::now().naive_utc();
    let size = checks.len();
    let mut errors = 0;
    for node in checks {
        let save = blacklist::ActiveModel {
            ip: Set(node.clone()),
            reason: Set("TOR Node".to_string()),
            expires_at: Set(None),
            created_at: Set(now),
            created_by: Set("tor-cron".to_string()),
            notes: Set("TOR Node, will not unban".to_string()),
        };

        match save.insert(db).await {
            Ok(_) => (),
            Err(_) => errors += 1
        }
    }

    let current_thread = thread::current();
    let thread_name = current_thread.name().unwrap_or("unknown");
    info!("Added {}/{} new TOR nodes to the blacklist on thread {}", size.clone() - errors, size, thread_name);
}

pub fn start(db: &DatabaseConnection) {

    static mut DATABASE: Option<DatabaseConnection> = None;
    unsafe {
        DATABASE = Some(db.clone());
    }

    let runner = every(1).day().at(10, 00, 00)
        .in_timezone(&Utc).perform(move || async {
        let database = unsafe { DATABASE.as_ref().unwrap() };
        get(database).await;
    });

    spawn(runner);

}
