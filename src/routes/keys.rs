use rocket::http::Status;
use rocket::serde::json::{Json, json};
use crate::structs::auth::{Auth, CreateKeyBody};
use crate::utils::response::DataResponse;

#[post("/keys/create", data = "<body>")]
pub async fn create_key(auth: Auth, body: Json<CreateKeyBody>) -> Result<Json<DataResponse>, Status> {
    if !auth.is_valid {
        return Err(Status::Unauthorized);
    }

    let master_key = std::env::var("MASTER_KEY").expect("MASTER_KEY must be set");
    if auth.key.unwrap() != master_key {
        return Err(Status::Unauthorized); // Only the master key can create keys
    }

    let response_json = DataResponse {
        status: "200".to_string(),
        data: json!({
            "key": "test",
        })
    };
    Ok(Json(response_json))

}