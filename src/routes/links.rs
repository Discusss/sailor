use rocket::http::Status;
use rocket::serde::json::{Json, Value};
use rocket::serde::json::serde_json::json;
use rocket::State;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use super::entities::{prelude::*, *};


#[get("/link?<domain>")]
pub async fn get_domain(db: &State<DatabaseConnection>, domain: String) -> Result<Json<Value>, Status> {
    let db = db as &DatabaseConnection;

    println!("Domain: {}", domain);

    let domain_info: links::Model = match Links::find()
        .filter(links::Column::Domain.contains(domain))
        .one(db)
        .await
    {
        Ok(domain_info) => match domain_info {
            Some(domain_info) => domain_info,
            None => return Err(Status::NotFound),
        },
        Err(_) => return Err(Status::NotFound),
    };

    let response_json = json!({
        "status": "success".to_string(),
        "data": json!({
            "domain": domain_info.domain,
            "category": domain_info.category,
            "priority": domain_info.priority,
            "notes": domain_info.public_notes,
            "submitted_by": domain_info.submitted_by,
            "submitted_at": domain_info.submitted_at,
        })
    });
    Ok(Json(response_json))

}