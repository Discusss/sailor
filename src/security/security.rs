use rocket::fairing;
use rocket::fairing::Fairing;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use crate::entities::blacklist;
use crate::entities::prelude::Blacklist;
use crate::structs::ip::get_ip;

pub struct Security {
    pub db: DatabaseConnection,
}

impl Security {
    pub fn new(db: &DatabaseConnection) -> Self {
        Self {
            db: db.clone()
        }
    }

}
#[rocket::async_trait]
impl Fairing for Security {

    fn info(&self) -> fairing::Info {
        fairing::Info {
            name: "Security middleware",
            kind: fairing::Kind::Request,
        }
    }

    async fn on_request(&self, request: &mut rocket::Request<'_>, _: &mut rocket::Data<'_>) {

        let path = request.uri().path();
        if path == "/api/blacklist/check" {
            return;
        }
        let ip = get_ip(request);
        match Blacklist::find()
            .filter(blacklist::Column::Ip.contains(ip.clone()))
            .one(&self.db)
            .await
        {
            Ok(ban) => match ban {
                Some(_) => {
                    println!("{} is blacklisted", ip);
                    
                },
                None => ()
            }
            Err(_) => ()
        }
    }
}