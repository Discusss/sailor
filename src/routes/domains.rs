use rocket::http::Status;
use rocket::serde::json::{Json};
use rocket::serde::json::serde_json::json;
use rocket::{State};
use rocket::response::status;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter};
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};
use crate::entities::{prelude::*, *};
use crate::structs::auth::{Auth, obfuscate_key};
use crate::structs::ip::RemoteAddress;
use crate::structs::types::LinkType;
use crate::utils::parser;
use crate::utils::response::DataResponse;
use crate::security::md5;

#[get("/domain?<domain>")]
pub async fn get_domain(db: &State<DatabaseConnection>, remote: RemoteAddress, domain: String, auth: Auth) -> Result<status::Custom<Json<DataResponse>>, Status> {
    let db = db as &DatabaseConnection;

    if remote.is_blacklisted {
        return Err(Status::Forbidden);
    }

    if !parser::is_valid_domain(&domain, db, &remote.ip).await {
        return Err(Status::BadRequest);
    }

    let domain_info: domains::Model = match Domains::find()
        .filter(domains::Column::Domain.contains(domain))
        .one(db)
        .await
    {
        Ok(domain_info) => match domain_info {
            Some(domain_info) => domain_info,
            None => return Err(Status::NotFound),
        },
        Err(_) => return Err(Status::InternalServerError),
    };

    let mut update: domains::ActiveModel = domain_info.clone().into();
    update.times_consulted = Set(domain_info.times_consulted + 1);
    match update.update(db).await {
        Ok(_) => {},
        Err(_) => {
            error!("Failed to update times_consulted for domain {}", domain_info.domain);
        },
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
            });
        }
        let response_json = DataResponse {
            status: "207".to_string(),
            data: json
        };
        return Ok(status::Custom(Status::from_code(207).unwrap(), Json(response_json)))

    } else {
        if !domain_info.approved_at.is_some() {
            return Err(Status::NotFound);
        }
    }

    let response_json = DataResponse {
        status: "200".to_string(),
        data: json
    };
    Ok(status::Custom(Status::from_code(200).unwrap(), Json(response_json)))
}

#[post("/domain", data="<body>")]
pub async fn create_domain(db: &State<DatabaseConnection>, remote: RemoteAddress, body: Json<CreateDomainBody>) -> Result<Json<DataResponse>, Status> {
    let db = db as &DatabaseConnection;

    if remote.is_blacklisted {
        return Err(Status::Forbidden);
    }

    if !parser::is_valid_domain(&body.domain, db, &remote.ip).await {
        return Err(Status::BadRequest);
    }

    match Domains::find()
        .filter(domains::Column::Domain.contains(body.domain.clone()))
        .one(db)
        .await
    {
        Ok(domain_info) => match domain_info {
            Some(_) => return Err(Status::Conflict),
            None => (),
        },
        Err(_) => return Err(Status::NotFound),
    };

    let now = chrono::Utc::now().naive_utc();
    let new_domain = domains::ActiveModel {
        domain: Set(body.domain.clone()),
        category: Set(body.category.unwrap_or(7)),
        priority: Set(body.priority.unwrap_or(0)),
        public_notes: Set("".to_string()),
        submitted_by: Set(body.submitted_by.clone()),
        submitted_at: Set(now),
        submitted_ip: Set(Some(remote.ip.clone())),
        submitted_user_agent: Set(remote.user_agent.clone()),
        submitted_reason: Set(body.reason.clone()),
        approved_by: Set(None),
        approved_at: Set(None),
        approved_key: Set(None),
        notes: Set(body.notes.clone().unwrap_or("".to_string())),
        times_consulted: Set(0),
        ..Default::default()
    };

    let new_domain = match new_domain.insert(db).await {
        Ok(domain) => domain,
        Err(_) => return Err(Status::InternalServerError),
    };

    let mut webhook_url = std::env::var("WEBHOOK_URL").expect("WEBHOOK_URL must be set");
    let webhook_secret = std::env::var("WEBHOOK_HASH_KEY").expect("WEBHOOK_HASH_KEY must be set");
    if webhook_url.ends_with("/") {
        webhook_url = webhook_url + "webhook"
    } else {
        webhook_url = webhook_url + "/webhook"
    }

    let webhook_data = json!({
            "id": new_domain.id,
            "domain": new_domain.domain,
            "category": LinkType::from_code(&new_domain.category).to_info(),
            "priority": new_domain.priority,
            "notes": new_domain.public_notes,
            "submitted_by": new_domain.submitted_by,
            "submitted_at": new_domain.submitted_at,
            "submitted_ip": new_domain.submitted_ip.unwrap_or("N/A".to_string()),
            "submitted_user_agent": new_domain.submitted_user_agent.unwrap_or("N/A".to_string()),
            "submitted_reason": new_domain.submitted_reason,
            "approved_by": new_domain.approved_by.unwrap_or("N/A".to_string()),
            "approved_at": new_domain.approved_at,
            "approved_key": new_domain.approved_key.unwrap_or("N/A".to_string()),
            "notes": new_domain.notes,
            "times_consulted": new_domain.times_consulted,
        });

    let hash = md5::compute(webhook_data.to_string() + &*webhook_secret);

    match ureq::post(&webhook_url)
        .set("Content-Type", "application/json")
        .set("User-Agent", "LA-CABRA Phishing API")
        .set("X-LACABRA-Signature", &*format!("{:x}", hash))
        .send_json(webhook_data) {
        Ok(_) => (),
        Err(e) => {
            log::error!("Error sending webhook: {}", e);
        },
    }

    let response_json = DataResponse {
        status: "200".to_string(),
        data: json!({
            "domain": new_domain.domain,
            "category": LinkType::from_code(&new_domain.category).to_info(),
            "priority": new_domain.priority,
            "notes": new_domain.public_notes,
            "submitted_by": new_domain.submitted_by,
            "submitted_at": new_domain.submitted_at,
        })
    };

    Ok(Json(response_json))
}

#[patch("/domain?<id>", data="<body>")]
pub async fn update_domain(db: &State<DatabaseConnection>, remote: RemoteAddress, auth: Auth, id: i32, body: Json<UpdateDomainBody>) -> Result<Json<DataResponse>, Status> {
    let db = db as &DatabaseConnection;

    if remote.is_blacklisted {
        return Err(Status::Forbidden);
    }

    if !auth.is_valid {
        return Err(Status::Unauthorized);
    }

    let mut update: domains::ActiveModel = match Domains::find()
        .filter(domains::Column::Id.eq(id))
        .one(db)
        .await
    {
        Ok(domain_info) => match domain_info {
            Some(domain_info) => domain_info.into(),
            None => return Err(Status::NotFound),
        },
        Err(_) => return Err(Status::NotFound),
    };

    let mut json = json!({});

    match &body.category {
        Some(category) => {
            update.category = Set(*category);
            json["category"] = json!(LinkType::from_code(category).to_info());
        }
        None => (),
    };

    match &body.priority {
        Some(priority) => {
            update.priority = Set(*priority);
            json["priority"] = json!(*priority);
        }
        None => (),
    };

    match &body.public_notes {
        Some(public_notes) => {
            update.public_notes = Set(public_notes.clone());
            json["public_notes"] = json!(public_notes.clone());
        }
        None => (),
    };

    match &body.submitted_by {
        Some(submitted_by) => {
            update.submitted_by = Set(submitted_by.clone());
            json["submitted_by"] = json!(submitted_by.clone());
        }
        None => (),
    };

    match &body.submitted_reason {
        Some(submitted_reason) => {
            update.submitted_reason = Set(submitted_reason.clone());
            json["submitted_reason"] = json!(submitted_reason.clone());
        }
        None => (),
    };

    match &body.approved_by {
        Some(approved_by) => {
            let now = chrono::Utc::now().naive_utc();
            let key = auth.key.unwrap();
            update.approved_by = Set(Some(approved_by.clone()));
            update.approved_at = Set(Some(now));
            update.approved_key = Set(Some(key.clone()));
            json["approved_by"] = json!(approved_by.clone());
            json["approved_at"] = json!(now);
            json["approved_key"] = json!(obfuscate_key(key));
        }
        None => (),
    };

    match &body.notes {
        Some(notes) => {
            update.notes = Set(notes.clone());
            json["notes"] = json!(notes.clone());
        }
        None => (),
    };

    match &body.times_consulted {
        Some(times_consulted) => {
            update.times_consulted = Set(*times_consulted);
            json["times_consulted"] = json!(*times_consulted);
        }
        None => (),
    };

    if json.is_null() || json == json!({}) {
        return Err(Status::BadRequest);
    }

    match update.update(db).await {
        Ok(_) => {},
        Err(_) => return Err(Status::InternalServerError),
    };

    let response_json = DataResponse {
        status: "200".to_string(),
        data: json
    };

    Ok(Json(response_json))
}

#[delete("/domain?<id>")]
pub async fn delete_domain(db: &State<DatabaseConnection>, remote: RemoteAddress, auth: Auth, id: i32) -> Result<Json<DataResponse>, Status> {
    let db = db as &DatabaseConnection;

    if remote.is_blacklisted {
        return Err(Status::Forbidden);
    }

    if !auth.is_valid {
        return Err(Status::Unauthorized);
    }

    let domain_to_delete = match Domains::find()
        .filter(domains::Column::Id.eq(id))
        .one(db)
        .await
    {
        Ok(domain_info) => match domain_info {
            Some(domain_info) => domain_info,
            None => return Err(Status::NotFound),
        },
        Err(_) => return Err(Status::InternalServerError),
    };

    let domain_info = domain_to_delete.clone();

    match domain_to_delete.delete(db).await {
        Ok(_) => (),
        Err(_) => return Err(Status::InternalServerError),
    };

    let response_json = DataResponse {
        status: "200".to_string(),
        data: json!({
                "id": domain_info.id,
                "domain": domain_info.domain,
                "category": LinkType::from_code(&domain_info.category).to_info(),
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
    };
    //todo: detect suspicious activity and blacklist the ip or ban the api key
    Ok(Json(response_json))
}

#[derive(Serialize, Deserialize)]
pub struct CreateDomainBody {
    pub domain: String,
    pub category: Option<i32>,
    pub priority: Option<i32>,
    pub notes: Option<String>,
    pub submitted_by: String,
    pub reason: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateDomainBody {
    pub category: Option<i32>,
    pub priority: Option<i32>,
    pub public_notes: Option<String>,
    pub submitted_by: Option<String>,
    pub submitted_reason: Option<String>,
    pub approved_by: Option<String>,
    pub notes: Option<String>,
    pub times_consulted: Option<i32>,
}