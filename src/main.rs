use std::process::exit;
use dotenvy::dotenv;
use rocket::http::Method;
use rocket::shield::Shield;
use migration::{Migrator, MigratorTrait};
use rocket_cors::AllowedHeaders;
use crate::structs::auth::validate_master_key;
use fern::colors::{Color, ColoredLevelConfig};

#[macro_use]
extern crate rocket;

extern crate fern;
#[macro_use]
extern crate log;
extern crate chrono;

mod db;
mod routes;
mod utils;
mod entities;
mod structs;

/*

    ==== Webhook ====
    POST /webhook (with JSON body) --> Mandar al bot de discord información sobre un nuevo dominio

 */

#[rocket::main]
async fn main() {
    setup_logger().expect("Failed to setup logger");
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = match db::set_up_db(database_url).await {
        Ok(db) => {
            assert!(db.ping().await.is_ok());
            info!("Database connection established");
            validate_master_key(&db).await;
            db
        },
        Err(err) => {
            error!("{}", err);
            exit(1);
        },
    };

    Migrator::up(&pool, None).await.unwrap();

    let cors = rocket_cors::CorsOptions {
        allowed_methods: vec![Method::Get, Method::Post, Method::Delete].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
        .to_cors();

    let prometheus = utils::prometheus::configure();

    if let Err(e) = rocket::build()
        .manage(pool)
        .mount("/api", routes::api())
        .mount("/stats", routes![routes::stats::get_stats])
        .mount("/metrics", prometheus.clone())
        .register("/", routes::catchers())
        .attach(Shield::default())
        .attach(cors.unwrap())
        .attach(prometheus)
        .launch()
        .await
    {
        error!("Error: {}", e);
    }
}

fn setup_logger() -> Result<(), fern::InitError> {

    let mut colors = ColoredLevelConfig::new();
    colors.warn = Color::Yellow;
    colors.info = Color::Green;
    colors.error = Color::Red;
    colors.debug = Color::BrightBlack;
    colors.trace = Color::Magenta;

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{}: {} {} - {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.target(),
                colors.color(record.level()),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}