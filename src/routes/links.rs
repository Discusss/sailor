use rocket::http::Status;
use rocket::serde::json::{Json, Value};
use rocket::serde::json::serde_json::json;
use rocket::State;
use sea_orm::DatabaseConnection;

#[get("/link?<domain>", format = "application/json")]
pub fn get_domain(db: &State<DatabaseConnection>, domain: String) -> Result<Json<Value>, Status> {

    let response_json = json!({
        "status": "success".to_string(),
        "message": "Domain found".to_string(),
    });
    Ok(Json(response_json))

}