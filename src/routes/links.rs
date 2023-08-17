use rocket::http::Status;
use rocket::serde::json::{Json, Value};
use rocket::serde::json::serde_json::json;
use rocket::{State};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use crate::entities::{prelude::*, *};
use crate::structs::ip::RemoteAddress;
use crate::utils::parser;

#[get("/link?<domain>")]
pub async fn get_domain(db: &State<DatabaseConnection>, remote: RemoteAddress, domain: String) -> Result<Json<Value>, Status> {
    let db = db as &DatabaseConnection;

    if !parser::is_valid_domain(&domain) {
        return Err(Status::BadRequest);
    }

    let ip = remote.0;

    match Blacklist::find()
        .filter(blacklist::Column::Ip.eq(ip))
        .one(db)
        .await
    {
        Ok(blacklisted_ip) => match blacklisted_ip {
            Some(_) => return Err(Status::Forbidden),
            _ => {}
        },
        Err(_) => return Err(Status::InternalServerError),
    };

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
        "status": "200".to_string(),
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