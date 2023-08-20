use rocket::http::Status;
use rocket::serde::json::{Json};
use rocket::serde::json::serde_json::json;
use rocket::{State};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use crate::entities::{prelude::*, *};
use crate::structs::auth::Auth;
use crate::structs::ip::RemoteAddress;
use crate::structs::types::LinkType;
use crate::utils::parser;
use crate::utils::response::DataResponse;

#[get("/link?<domain>")]
pub async fn get_domain(db: &State<DatabaseConnection>, remote: RemoteAddress, domain: String, auth: Auth) -> Result<Json<DataResponse>, Status> {
    let db = db as &DatabaseConnection;

    if !parser::is_valid_domain(&domain) {
        return Err(Status::BadRequest);
    }

    if remote.is_blacklisted {
        return Err(Status::Forbidden);
    }

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

    let category = LinkType::from_code(&domain_info.category).to_info();
    let mut json = json!({
            "domain": domain_info.domain,
            "category": category,
            "priority": domain_info.priority,
            "notes": domain_info.public_notes,
            "submitted_by": domain_info.submitted_by,
            "submitted_at": domain_info.submitted_at,
        });

    if auth.is_valid {
        if auth.is_master {
            json = json!({
                "id": domain_info.id,
                "domain": domain_info.domain,
                "category": category,
                "priority": domain_info.priority,
                "public_notes": domain_info.public_notes,
                "submitted_by": domain_info.submitted_by,
                "submitted_at": domain_info.submitted_at,
                "submitted_ip": domain_info.submitted_ip.unwrap_or("N/A".to_string()),
                "submitted_user_agent": domain_info.submitted_user_agent.unwrap_or("N/A".to_string()),
                "submitted_reason": domain_info.submitted_reason,
                "approved_by": domain_info.approved_by.unwrap_or("N/A".to_string()),
                "approved_at": domain_info.approved_at,
                "approved_key": domain_info.approved_key.unwrap_or("N/A".to_string()),
                "notes": domain_info.notes,
                "times_consulted": domain_info.times_consulted,
            })
        } else {
            json = json!({
                "id": domain_info.id,
                "domain": domain_info.domain,
                "category": category,
                "priority": domain_info.priority,
                "public_notes": domain_info.public_notes,
                "submitted_by": domain_info.submitted_by,
                "submitted_at": domain_info.submitted_at,
                "submitted_reason": domain_info.submitted_reason,
                "approved_by": domain_info.approved_by.unwrap_or("N/A".to_string()),
                "approved_at": domain_info.approved_at,
                "notes": domain_info.notes,
                "times_consulted": domain_info.times_consulted,
            })
        }
    }

    let response_json = DataResponse {
        status: "200".to_string(),
        data: json
    };
    Ok(Json(response_json))
}