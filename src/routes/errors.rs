use rocket::http::Status;
use rocket::Request;
use rocket::serde::json::Json;
use crate::utils::response::GenericResponse;

#[catch(401)]
pub fn unauthorized() -> Json<GenericResponse> {
    let response_json = GenericResponse {
        status: "401".to_string(),
        message: "Unauthorized".to_string(),
    };
    Json(response_json)
}

#[catch(403)]
pub fn forbidden() -> Json<GenericResponse> {
    let response_json = GenericResponse {
        status: "403".to_string(),
        message: "You are forbidden from accessing this resource. You can check your blacklist status at /api/blacklist/check".to_string(),
    };
    Json(response_json)
}

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