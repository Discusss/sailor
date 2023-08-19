use rocket::serde::json::{Value};
use serde::Serialize;

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[derive(Serialize)]
pub struct DataResponse {
    pub status: String,
    pub data: Value
}

#[derive(Serialize)]
pub struct DataResponseArray {
    pub status: String,
    pub data: Vec<Value>
}
