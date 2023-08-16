use rocket::http::Status;
use rocket::serde::json::{Json, Value};
use rocket::serde::json::serde_json::json;
use crate::links_model::Links;

#[get("/link?<domain>", format = "application/json")]
pub fn get_domain(domain: String) -> Result<Json<Value>, Status> {

    let conn = crate::db::establish_connection();

    let info = Links::all(&conn.get().unwrap());

    if info.is_empty() {
        return Err(Status::NotFound);
    }

    let response_json = json!({
        "status": "success".to_string(),
        "message": info,
    });
    Ok(Json(response_json))

}