#![feature(async_closure)]

use std::process::exit;
use std::thread;
use dotenvy::dotenv;
use rocket::http::Method;
use rocket::shield::Shield;
use migration::{Migrator, MigratorTrait};
use rocket_cors::AllowedHeaders;
use crate::structs::auth::validate_master_key;

#[macro_use]
extern crate rocket;

extern crate fern;
#[macro_use]
extern crate log;
extern crate chrono;
extern crate io;

mod db;
mod routes;
mod utils;
mod entities;
mod structs;
pub(crate) mod security;
mod logger;

#[rocket::main]
async fn main() {

    logger::source::setup_logger().expect("Failed to setup logger");
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = match db::set_up_db(database_url).await {
        Ok(db) => {
            assert!(db.ping().await.is_ok());
            info!("Database connection established");
            Migrator::up(&db, None).await.unwrap();
            validate_master_key(&db).await;
            db
        },
        Err(err) => {
            error!("{}", err);
            exit(1);
        },
    };

    let db = pool.clone();
    thread::spawn(async move || {
        info!("Starting TOR IP download cron job");
        security::tor::get(&db).await;
        security::tor::start(&db);
    }).join().unwrap().await;

    let cors = rocket_cors::CorsOptions {
        allowed_methods: vec![Method::Get, Method::Post, Method::Delete].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
        .to_cors();

    let prometheus = utils::prometheus::configure();
    //let security = security::security::Security::new(&pool);

    if let Err(e) = rocket::build()
        .manage(pool)
        .mount("/api", routes::api())
        .mount("/stats", routes![routes::stats::get_stats])
        .mount("/metrics", prometheus.clone())
        .register("/", routes::catchers())
        .attach(Shield::default())
        .attach(cors.unwrap())
        .attach(prometheus)
        //.attach(security)
        .launch()
        .await
    {
        error!("Error: {}", e);
    }
}