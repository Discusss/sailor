use dotenvy::dotenv;
use rocket::{get, http::Status, serde::json::Json};
use serde::Serialize;

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate log;

mod db;
mod schema;

#[path = "routes/links.rs"]
mod links_router;

#[path = "models/links.rs"]
mod links_model;


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

#[get("/healthchecker", format = "application/json")]
pub async fn health_checker_handler() -> Result<Json<GenericResponse>, Status> {
    const MESSAGE: &str = "Build Simple CRUD API with Rust and Rocket";

    let response_json = GenericResponse {
        status: "success".to_string(),
        message: MESSAGE.to_string(),
    };
    Ok(Json(response_json))
}

#[catch(404)]
pub fn not_found() -> Json<GenericResponse> {
    let response_json = GenericResponse {
        status: "error".to_string(),
        message: "Not found".to_string(),
    };
    Json(response_json)
}

#[catch(default)]
pub fn internal_error() -> Json<GenericResponse> {
    let response_json = GenericResponse {
        status: "error".to_string(),
        message: "Internal server error".to_string(),
    };
    Json(response_json)
}

#[rocket::main]
async fn main() {
    pretty_env_logger::init();

    dotenv().ok();
    let pool = db::establish_connection();

    if let Err(e) = rocket::build()
        .manage(pool)
        .mount("/api", routes![health_checker_handler, links_router::get_domain])
        .register("/", catchers![not_found, internal_error])
        .launch()
        .await
    {
        error!("Error: {}", e);
    }
}