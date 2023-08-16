use std::process::exit;
use dotenvy::dotenv;
use rocket::{serde::json::Json};
use serde::Serialize;

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate log;

mod db;

#[path = "routes/links.rs"]
mod links_router;



#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

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

#[catch(404)]
pub fn not_found() -> Json<GenericResponse> {
    let response_json = GenericResponse {
        status: "404".to_string(),
        message: "Not found".to_string(),
    };
    Json(response_json)
}

#[catch(default)]
pub fn internal_error() -> Json<GenericResponse> {
    let response_json = GenericResponse {
        status: "500".to_string(),
        message: "Internal server error".to_string(),
    };
    Json(response_json)
}

#[rocket::main]
async fn main() {
    pretty_env_logger::init();
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = match db::set_up_db(database_url).await {
        Ok(db) =>{
            assert!(db.ping().await.is_ok());
            info!("Database connection established");
            db
        },
        Err(err) => {
            error!("{}", err);
            exit(1);
        },
    };

    if let Err(e) = rocket::build()
        .manage(pool)
        .mount("/api", routes![links_router::get_domain])
        .register("/", catchers![not_found, internal_error])
        .launch()
        .await
    {
        error!("Error: {}", e);
    }
}