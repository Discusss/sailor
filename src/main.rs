use std::process::exit;
use dotenvy::dotenv;
use rocket::http::Method;
use rocket::shield::Shield;
use migration::{Migrator, MigratorTrait};
use rocket_cors::AllowedHeaders;

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate log;

mod db;
mod routes;
mod utils;
mod entities;
mod structs;

/*

    GET /api/link?domain=example.com --> Comprobar si existe el dominio en la base de datos
    POST /api/link (with JSON body) --> Insertar el dominio en la base de datos (pero sin aprobar)
    POST /api/link/approve (with JSON body, needs auth) --> Aprobar el dominio en la base de datos
    POST /api/link/reject (with JSON body, needs auth) --> Rechazar el dominio en la base de datos (borrarlo)

    GET /api/blacklist (needs auth) --> Obtener todas las IPs de la lista negra
    POST /api/blacklist (with JSON body, needs auth) --> Insertar una IP en la lista negra

    GET /docs/... --> Documentación de la API

    ==== Webhook ====
    POST /webhook (with JSON body) --> Mandar al bot de discord información sobre un nuevo dominio

 */

#[rocket::main]
async fn main() {
    pretty_env_logger::init();
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = match db::set_up_db(database_url).await {
        Ok(db) => {
            assert!(db.ping().await.is_ok());
            info!("Database connection established");
            db
        },
        Err(err) => {
            error!("{}", err);
            exit(1);
        },
    };

    Migrator::up(&pool, None).await.unwrap();

    let cors = rocket_cors::CorsOptions {
        allowed_methods: vec![Method::Get, Method::Post].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
        .to_cors();

    let prometheus = utils::prometheus::configure();

    if let Err(e) = rocket::build()
        .manage(pool)
        .mount("/api", routes![routes::links::get_domain])
        .mount("/metrics", prometheus.clone())
        .register("/", catchers![routes::errors::not_found, routes::errors::internal_error])
        .attach(Shield::default())
        .attach(cors.unwrap())
        .attach(prometheus)
        .launch()
        .await
    {
        error!("Error: {}", e);
    }
}