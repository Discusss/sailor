use rocket::http::Status;
use rocket::serde::json::{Json, json};
use rocket::State;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter};
use crate::entities::keys;
use crate::entities::prelude::Keys;
use crate::structs::auth::{ApiKey, Auth, CreateKeyBody, is_valid_key, UpdateKeyBody};
use crate::utils::response::DataResponse;

#[post("/keys/create", data = "<body>")]
pub async fn create_key(db: &State<DatabaseConnection>, auth: Auth, body: Json<CreateKeyBody>) -> Result<Json<DataResponse>, Status> {
    let db = db as &DatabaseConnection;

    if !auth.is_valid {
        return Err(Status::Unauthorized);
    }

    let master_key = std::env::var("MASTER_KEY").expect("MASTER_KEY must be set");
    if auth.key.unwrap() != master_key {
        return Err(Status::Unauthorized); // Only the master key can create keys
    }

    let expires_at = chrono::DateTime::parse_from_rfc3339(&body.expires_at);
    if expires_at.is_err() {
        return Err(Status::BadRequest);
    }
    if expires_at.unwrap().naive_utc() < chrono::Utc::now().naive_utc() || expires_at.unwrap().naive_utc() > chrono::Utc::now().naive_utc() + chrono::Duration::days(365) {
        return Err(Status::BadRequest);
    }

    let now = chrono::Utc::now().naive_utc();
    let key = keys::ActiveModel {
        key: Set(ApiKey::generate()),
        created_at: Set(now.clone()),
        expires_at: Set(Some(expires_at.unwrap().naive_utc())),
        last_used_at: Set(None),
        owner: Set(body.owner.clone()),
        uses: Set(0),
        ips: Set(vec![]),
        user_agent: Set("system".to_string()),
        created_by: Set(body.created_by.clone()),
        notes: Set(body.notes.clone().unwrap_or("".to_string())),
    };

    let key = match key.insert(db).await {
        Ok(key) => key,
        Err(err) => {
            error!("Error creating key: {}", err);
            return Err(Status::InternalServerError);
        },
    };

    let response_json = DataResponse {
        status: "200".to_string(),
        data: json!({
            "key": key.key,
            "created_at": key.created_at,
            "expires_at": key.expires_at,
            "owner": key.owner,
            "created_by": key.created_by,
            "notes": key.notes,
        })
    };
    Ok(Json(response_json))

}

#[post("/keys/update?<key>", data = "<body>")]
pub async fn update_key(db: &State<DatabaseConnection>, auth: Auth, key: String, body: Json<UpdateKeyBody>) -> Result<Json<DataResponse>, Status> {
    let db = db as &DatabaseConnection;

    if !auth.is_valid {
        return Err(Status::Unauthorized);
    }

    let master_key = std::env::var("MASTER_KEY").expect("MASTER_KEY must be set");
    if auth.key.unwrap() != master_key {
        return Err(Status::Unauthorized); // Only the master key can update keys
    }

    if !is_valid_key(&key) {
        return Err(Status::BadRequest);
    }

    if key == master_key {
        return Err(Status::BadRequest); // The master key cannot be updated
    }

    let mut update: keys::ActiveModel = match Keys::find()
        .filter(keys::Column::Key.eq(key.clone()))
        .one(db)
        .await
    {
        Ok(key_info) => match key_info {
            Some(key_info) => key_info.into(),
            None => return Err(Status::NotFound),
        },
        Err(_) => return Err(Status::NotFound),
    };

    let mut updated = 0;
    let mut json = json!({});

    match &body.expires_at {
        Some(expires_at) => {
            let expires_at = chrono::DateTime::parse_from_rfc3339(&expires_at);
            if expires_at.is_err() {
                return Err(Status::BadRequest);
            }
            if expires_at.unwrap().naive_utc() < chrono::Utc::now().naive_utc() || expires_at.unwrap().naive_utc() > chrono::Utc::now().naive_utc() + chrono::Duration::days(365) {
                return Err(Status::BadRequest);
            }
            update.expires_at = Set(Some(expires_at.unwrap().naive_utc()));
            updated += 1;
            json["expires_at"] = body.expires_at.clone().unwrap().into();
        },
        None => (),
    };

    match &body.last_used_at {
        Some(last_used_at) => {
            let last_used_at = chrono::DateTime::parse_from_rfc3339(&last_used_at);
            if last_used_at.is_err() {
                return Err(Status::BadRequest);
            }
            if update.created_at.unwrap() > last_used_at.unwrap().naive_utc() {
                return Err(Status::BadRequest);
            }
            update.last_used_at = Set(Some(last_used_at.unwrap().naive_utc()));
            updated += 1;
            json["last_used_at"] = body.last_used_at.clone().unwrap().into();
        },
        None => (),
    };

    match &body.owner {
        Some(owner) => {
            update.owner = Set(owner.clone());
            updated += 1;
            json["owner"] = body.owner.clone().unwrap().into();
        },
        None => (),
    };

    match &body.uses {
        Some(uses) => {
            update.uses = Set(uses.clone());
            updated += 1;
            json["uses"] = body.uses.clone().unwrap().into();
        },
        None => (),
    };

    match &body.ips {
        Some(ips) => {
            update.ips = Set(ips.clone());
            updated += 1;
            json["ips"] = body.ips.clone().unwrap().into();
        },
        None => (),
    };

    match &body.user_agent {
        Some(user_agent) => {
            update.user_agent = Set(user_agent.clone());
            updated += 1;
            json["user_agent"] = body.user_agent.clone().unwrap().into();
        },
        None => (),
    };

    match &body.created_by {
        Some(created_by) => {
            update.created_by = Set(created_by.clone());
            updated += 1;
            json["created_by"] = body.created_by.clone().unwrap().into();
        },
        None => (),
    };

    match &body.notes {
        Some(notes) => {
            update.notes = Set(notes.clone());
            updated += 1;
            json["notes"] = body.notes.clone().unwrap().into();
        },
        None => (),
    };

    if updated == 0 {
        return Err(Status::BadRequest);
    }

    let response_json = DataResponse {
        status: "200".to_string(),
        data: json
    };

    Ok(Json(response_json))

}

#[get("/keys?<key>")]
pub async fn get_key(db: &State<DatabaseConnection>, auth: Auth, key: String) -> Result<Json<DataResponse>, Status> {
    let db = db as &DatabaseConnection;

    if !auth.is_valid {
        return Err(Status::Unauthorized);
    }

    let master_key = std::env::var("MASTER_KEY").expect("MASTER_KEY must be set");
    if auth.key.unwrap() != master_key {
        return Err(Status::Unauthorized); // Only the master key can check keys
    }

    if !is_valid_key(&key) {
        return Err(Status::BadRequest);
    }

    let key_info: keys::Model = match Keys::find()
        .filter(keys::Column::Key.eq(key.clone()))
        .one(db)
        .await
    {
        Ok(key_info) => match key_info {
            Some(key_info) => key_info,
            None => return Err(Status::NotFound),
        },
        Err(_) => return Err(Status::NotFound),
    };

    let response_json = DataResponse {
        status: "200".to_string(),
        data: json!({
            "key": key_info.key,
            "created_at": key_info.created_at,
            "expires_at": key_info.expires_at,
            "last_used_at": key_info.last_used_at,
            "owner": key_info.owner,
            "uses": key_info.uses,
            "ips": key_info.ips,
            "user_agent": key_info.user_agent,
            "created_by": key_info.created_by,
            "notes": key_info.notes,
        })
    };
    Ok(Json(response_json))

}

#[delete("/keys?<key>")]
pub async fn delete_key(db: &State<DatabaseConnection>, auth: Auth, key: String) -> Result<Json<DataResponse>, Status> {
    let db = db as &DatabaseConnection;

    if !auth.is_valid {
        return Err(Status::Unauthorized);
    }

    let master_key = std::env::var("MASTER_KEY").expect("MASTER_KEY must be set");
    if auth.key.unwrap() != master_key {
        return Err(Status::Unauthorized); // Only the master key can delete keys
    }

    if !is_valid_key(&key) {
        return Err(Status::BadRequest);
    }

    if key == master_key {
        return Err(Status::BadRequest); // The master key cannot be deleted
    }

    let key_to_delete = match Keys::find()
        .filter(keys::Column::Key.eq(key.clone()))
        .one(db)
        .await
    {
        Ok(key_info) => match key_info {
            Some(key_info) => key_info,
            None => return Err(Status::NotFound),
        },
        Err(_) => return Err(Status::NotFound),
    };

    let key_info = key_to_delete.clone();

    match key_to_delete.delete(db).await
    {
        Ok(_) => (),
        Err(_) => return Err(Status::InternalServerError),
    };

    let response_json = DataResponse {
        status: "200".to_string(),
        data: json!({
            "key": key_info.key,
            "created_at": key_info.created_at,
            "expires_at": key_info.expires_at,
            "last_used_at": key_info.last_used_at,
            "owner": key_info.owner,
            "uses": key_info.uses,
            "ips": key_info.ips,
            "user_agent": key_info.user_agent,
            "created_by": key_info.created_by,
            "notes": key_info.notes,
        })
    };
    Ok(Json(response_json))

}