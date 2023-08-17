use rocket::http::Status;
use rocket::Request;
use rocket::serde::json::Json;
use crate::utils::response::GenericResponse;

#[catch(404)]
pub fn not_found() -> Json<GenericResponse> {
    let response_json = GenericResponse {
        status: "404".to_string(),
        message: "Not found".to_string(),
    };
    Json(response_json)
}

#[catch(default)]
pub fn internal_error(status: Status, request: &Request) -> Json<GenericResponse> {
    let response_json = GenericResponse {
        status: status.code.to_string(),
        message: format!("{}: '{}'", status, request.uri()),
    };
    Json(response_json)
}