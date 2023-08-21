use std::net::IpAddr;
use rocket::http::Status;
use rocket::serde::json::{Json, json};
use rocket::State;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter};
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};
use crate::entities::blacklist;
use crate::entities::prelude::Blacklist;
use crate::structs::auth::Auth;
use crate::utils::response::{DataResponse, DataResponseArray};
use validators::prelude::*;
use crate::structs::ip::RemoteAddress;

#[get("/blacklist")]
pub async fn get_all_blacklist(db: &State<DatabaseConnection>, auth: Auth) -> Result<Json<DataResponseArray>, Status> {
    let db = db as &DatabaseConnection;

    if !auth.is_valid || !auth.is_master {
        return Err(Status::Unauthorized);
    }

    let bans = match Blacklist::find().all(db).await {
        Ok(bans) => bans,
        Err(_) => return Err(Status::InternalServerError),
    };

    let response_json = DataResponseArray {
        status: "200".to_string(),
        data: bans.into_iter().map(|ban| {
            json!({
                "ip": ban.ip,
                "reason": ban.reason,
            })
        }).collect()
    };

    Ok(Json(response_json))
}

#[get("/blacklist?<ip>")]
pub async fn get_blacklist(db: &State<DatabaseConnection>, auth: Auth, ip: String) -> Result<Json<DataResponse>, Status> {
    let db = db as &DatabaseConnection;

    if !auth.is_valid || !auth.is_master {
        return Err(Status::Unauthorized);
    }

    let ban = match Blacklist::find()
        .filter(blacklist::Column::Ip.contains(ip))
        .one(db)
        .await
    {
        Ok(ban) => match ban {
            Some(ban) => ban,
            None => return Err(Status::NotFound),
        },
        Err(_) => return Err(Status::NotFound),
    };

    let response_json = DataResponse {
        status: "200".to_string(),
        data: json!({
            "ip": ban.ip,
            "reason": ban.reason,
            "expires_at": ban.expires_at,
            "created_at": ban.created_at,
            "created_by": ban.created_by,
            "notes": ban.notes,
        })
    };

    Ok(Json(response_json))
}

#[get("/blacklist/check")]
pub async fn check_blacklist(db: &State<DatabaseConnection>, remote: RemoteAddress)-> Result<Json<DataResponse>, Status> {
    let db = db as &DatabaseConnection;

    if !remote.is_blacklisted {
        return Err(Status::NotFound);
    }

    let ip = remote.ip;

    let ban = match Blacklist::find()
        .filter(blacklist::Column::Ip.contains(ip))
        .one(db)
        .await
    {
        Ok(ban) => match ban {
            Some(ban) => ban,
            None => return Err(Status::NotFound),
        },
        Err(_) => return Err(Status::NotFound),
    };

    let response_json = DataResponse {
        status: "200".to_string(),
        data: json!({
            "ip": ban.ip,
            "reason": ban.reason,
            "expires_at": ban.expires_at,
            "created_at": ban.created_at,
            "created_by": ban.created_by,
            "notes": ban.notes,
        })
    };

    Ok(Json(response_json))
}

#[post("/blacklist", data = "<body>")]
pub async fn create_blacklist(db: &State<DatabaseConnection>, auth: Auth, body: Json<CreateBlacklistBody>) -> Result<Json<DataResponse>, Status> {
    let db = db as &DatabaseConnection;

    if !auth.is_valid || !auth.is_master {
        return Err(Status::Unauthorized);
    }

    let expires_at = match body.expires_at.clone() {
        Some(expires_at) => match chrono::DateTime::parse_from_rfc3339(&expires_at) {
            Ok(expires_at) => {
                if expires_at.naive_utc() < chrono::Utc::now().naive_utc() || expires_at.naive_utc() > chrono::Utc::now().naive_utc() + chrono::Duration::days(365) {
                    return Err(Status::BadRequest);
                }
                Some(expires_at.naive_utc())
            },
            Err(_) => return Err(Status::BadRequest),
        },
        None => None,
    };

    if !IP::parse_string(&body.ip).is_ok() {
        return Err(Status::BadRequest);
    }

    match Blacklist::find()
        .filter(blacklist::Column::Ip.contains(body.ip.clone()))
        .one(db)
        .await
    {
        Ok(ban) => match ban {
            Some(_) => return Err(Status::Conflict),
            None => (),
        },
        Err(_) => return Err(Status::InternalServerError),
    };

    let now = chrono::Utc::now().naive_utc();
    let ban = blacklist::ActiveModel {
        ip: Set(body.ip.clone()),
        reason: Set(body.reason.clone()),
        expires_at: Set(expires_at),
        created_at: Set(now),
        created_by: Set(body.created_by.clone()),
        notes: Set(body.notes.clone().unwrap_or("".to_string())),
    };

    let ban = match ban.insert(db).await {
        Ok(ban) => ban,
        Err(err) => {
            error!("Error creating blacklist entry: {}", err);
            return Err(Status::InternalServerError)
        },
    };

    let response_json = DataResponse {
        status: "200".to_string(),
        data: json!({
            "ip": ban.ip,
            "reason": ban.reason,
            "expires_at": ban.expires_at,
            "created_at": ban.created_at,
            "created_by": ban.created_by,
            "notes": ban.notes,
        })
    };

    Ok(Json(response_json))
}

#[patch("/blacklist?<ip>", data = "<body>")]
pub async fn update_blacklist(db: &State<DatabaseConnection>, auth: Auth, ip: String, body: Json<UpdateBlacklistBody>) -> Result<Json<DataResponse>, Status> {
    let db = db as &DatabaseConnection;

    if !auth.is_valid || !auth.is_master {
        return Err(Status::Unauthorized);
    }

    if !IP::parse_string(&ip).is_ok() {
        return Err(Status::BadRequest);
    }

    let mut update: blacklist::ActiveModel = match Blacklist::find()
        .filter(blacklist::Column::Ip.contains(ip))
        .one(db)
        .await
    {
        Ok(ban) => match ban {
            Some(ban) => ban.into(),
            None => return Err(Status::NotFound),
        },
        Err(_) => return Err(Status::NotFound),
    };

    let mut json = json!({});

    match &body.reason {
        Some(reason) => {
            update.reason = Set(reason.clone());
            json["reason"] = json!(reason.clone());
        },
        None => {}
    };

    match &body.expires_at {
        Some(expires_at) => {
            match chrono::DateTime::parse_from_rfc3339(&expires_at) {
                Ok(expires_at) => {
                    if expires_at.naive_utc() < chrono::Utc::now().naive_utc() || expires_at.naive_utc() > chrono::Utc::now().naive_utc() + chrono::Duration::days(365) {
                        return Err(Status::BadRequest);
                    }
                    update.expires_at = Set(Some(expires_at.naive_utc()));
                    json["expires_at"] = json!(expires_at.naive_utc());
                },
                Err(_) => return Err(Status::BadRequest),
            };
        },
        None => {}
    };

    match &body.created_by {
        Some(created_by) => {
            update.created_by = Set(created_by.clone());
            json["created_by"] = json!(created_by.clone());
        },
        None => {}
    };

    match &body.notes {
        Some(notes) => {
            update.notes = Set(notes.clone());
            json["notes"] = json!(notes.clone());
        },
        None => {}
    };

    if json == json!({}) {
        return Err(Status::BadRequest);
    }

    match update.update(db).await {
        Ok(_) => {},
        Err(err) => {
            error!("Error updating blacklist entry: {}", err);
            return Err(Status::InternalServerError)
        },
    };

    let response_json = DataResponse {
        status: "200".to_string(),
        data: json
    };

    Ok(Json(response_json))
}

#[delete("/blacklist?<ip>")]
pub async fn delete_blacklist(db: &State<DatabaseConnection>, auth: Auth, ip: String) -> Result<Json<DataResponse>, Status> {
    let db = db as &DatabaseConnection;

    if !auth.is_valid || !auth.is_master {
        return Err(Status::Unauthorized);
    }

    if !IP::parse_string(&ip).is_ok() {
        return Err(Status::BadRequest);
    }

    let ban_to_delete = match Blacklist::find()
        .filter(blacklist::Column::Ip.contains(ip))
        .one(db)
        .await
    {
        Ok(ban) => match ban {
            Some(ban) => ban,
            None => return Err(Status::NotFound),
        },
        Err(_) => return Err(Status::NotFound),
    };

    let ban_info = ban_to_delete.clone();

    match ban_to_delete.delete(db).await
    {
        Ok(_) => {},
        Err(err) => {
            error!("Error deleting blacklist entry: {}", err);
            return Err(Status::InternalServerError)
        },
    };

    let response_json = DataResponse {
        status: "200".to_string(),
        data: json!({
            "ip": ban_info.ip,
            "reason": ban_info.reason,
            "expires_at": ban_info.expires_at,
            "created_at": ban_info.created_at,
            "created_by": ban_info.created_by,
            "notes": ban_info.notes,
        })
    };

    Ok(Json(response_json))

}

#[derive(Validator)]
#[validator(ip(local(NotAllow), port(NotAllow)))]
pub struct IP(IpAddr);

#[derive(Serialize, Deserialize)]
pub struct CreateBlacklistBody {
    ip: String,
    reason: String,
    expires_at: Option<String>,
    created_by: String,
    notes: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateBlacklistBody {
    reason: Option<String>,
    expires_at: Option<String>,
    created_by: Option<String>,
    notes: Option<String>,
}