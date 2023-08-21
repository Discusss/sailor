use rocket::http::Status;
use rocket::serde::json::{Json, json};
use rocket::State;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use crate::entities::keys;
use crate::entities::prelude::Keys;
use crate::structs::auth::{ApiKey, Auth, is_valid_key};
use crate::utils::response::DataResponse;

#[get("/keys?<key>")]
pub async fn get_key(db: &State<DatabaseConnection>, auth: Auth, key: String) -> Result<Json<DataResponse>, Status> {
    let db = db as &DatabaseConnection;

    if !auth.is_valid || !auth.is_master {
        return Err(Status::Unauthorized);
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

#[post("/keys", data = "<body>")]
pub async fn create_key(db: &State<DatabaseConnection>, auth: Auth, body: Json<CreateKeyBody>) -> Result<Json<DataResponse>, Status> {
    let db = db as &DatabaseConnection;

    if !auth.is_valid || !auth.is_master {
        return Err(Status::Unauthorized);
    }

    let expires_at = match chrono::DateTime::parse_from_rfc3339(&body.expires_at) {
        Ok(expires_at) => {
            if expires_at.naive_utc() < chrono::Utc::now().naive_utc() || expires_at.naive_utc() > chrono::Utc::now().naive_utc() + chrono::Duration::days(365*5) {
                return Err(Status::BadRequest);
            }
            expires_at
        },
        Err(_) => return Err(Status::BadRequest)
    };

    let now = chrono::Utc::now().naive_utc();
    let key = keys::ActiveModel {
        key: Set(ApiKey::generate()),
        created_at: Set(now.clone()),
        expires_at: Set(Some(expires_at.naive_utc())),
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

#[patch("/keys?<key>", data = "<body>")]
pub async fn update_key(db: &State<DatabaseConnection>, auth: Auth, key: String, body: Json<UpdateKeyBody>) -> Result<Json<DataResponse>, Status> {
    let db = db as &DatabaseConnection;

    if !auth.is_valid || !auth.is_master {
        return Err(Status::Unauthorized);
    }

    if !is_valid_key(&key) {
        return Err(Status::BadRequest);
    }

    let master_key = std::env::var("MASTER_KEY").expect("MASTER_KEY must be set");
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
        Err(_) => return Err(Status::InternalServerError),
    };

    let mut json = json!({});

    match &body.expires_at {
        Some(expires_at) => {
            match chrono::DateTime::parse_from_rfc3339(&expires_at) {
                Ok(expires_at) => {
                    if expires_at.naive_utc() < chrono::Utc::now().naive_utc() || expires_at.naive_utc() > chrono::Utc::now().naive_utc() + chrono::Duration::days(365) {
                        return Err(Status::BadRequest);
                    }
                    update.expires_at = Set(Some(expires_at.naive_utc()));
                    json["expires_at"] = body.expires_at.clone().unwrap().into();
                },
                Err(_) => return Err(Status::BadRequest)
            };
        },
        None => (),
    };

    match &body.last_used_at {
        Some(last_used_at) => {
            match chrono::DateTime::parse_from_rfc3339(&last_used_at) {
                Ok(last_used_at) => {
                    if last_used_at.naive_utc() < update.clone().created_at.unwrap() || last_used_at.naive_utc() > chrono::Utc::now().naive_utc() {
                        return Err(Status::BadRequest);
                    }
                    update.last_used_at = Set(Some(last_used_at.naive_utc()));
                    json["last_used_at"] = body.last_used_at.clone().unwrap().into();
                },
                Err(_) => return Err(Status::BadRequest)
            };
        },
        None => (),
    };

    match &body.owner {
        Some(owner) => {
            update.owner = Set(owner.clone());
            json["owner"] = body.owner.clone().unwrap().into();
        },
        None => (),
    };

    match &body.uses {
        Some(uses) => {
            update.uses = Set(uses.clone());
            json["uses"] = body.uses.clone().unwrap().into();
        },
        None => (),
    };

    match &body.ips {
        Some(ips) => {
            update.ips = Set(ips.clone());
            json["ips"] = body.ips.clone().unwrap().into();
        },
        None => (),
    };

    match &body.user_agent {
        Some(user_agent) => {
            update.user_agent = Set(user_agent.clone());
            json["user_agent"] = body.user_agent.clone().unwrap().into();
        },
        None => (),
    };

    match &body.created_by {
        Some(created_by) => {
            update.created_by = Set(created_by.clone());
            json["created_by"] = body.created_by.clone().unwrap().into();
        },
        None => (),
    };

    match &body.notes {
        Some(notes) => {
            update.notes = Set(notes.clone());
            json["notes"] = body.notes.clone().unwrap().into();
        },
        None => (),
    };

    if json == json!({}) {
        return Err(Status::BadRequest);
    }

    match update.update(db).await {
        Ok(_) => (),
        Err(err) => {
            error!("Error updating key: {}", err);
            return Err(Status::InternalServerError);
        },
    };

    let response_json = DataResponse {
        status: "200".to_string(),
        data: json
    };

    Ok(Json(response_json))

}

#[delete("/keys?<key>")]
pub async fn delete_key(db: &State<DatabaseConnection>, auth: Auth, key: String) -> Result<Json<DataResponse>, Status> {
    let db = db as &DatabaseConnection;

    if !auth.is_valid || !auth.is_master {
        return Err(Status::Unauthorized);
    }

    if !is_valid_key(&key) {
        return Err(Status::BadRequest);
    }

    let master_key = std::env::var("MASTER_KEY").expect("MASTER_KEY must be set");
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
        Err(_) => return Err(Status::InternalServerError),
    };

    let key_info = key_to_delete.clone();

    match key_to_delete.delete(db).await
    {
        Ok(_) => (),
        Err(err) => {
            error!("Error deleting key: {}", err);
            return Err(Status::InternalServerError)
        },
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

#[derive(Serialize, Deserialize)]
pub struct CreateKeyBody {
    pub(crate) expires_at: String,
    pub(crate) owner: String,
    pub(crate) created_by: String,
    pub(crate) notes: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateKeyBody {
    pub(crate) expires_at: Option<String>,
    pub(crate) last_used_at: Option<String>,
    pub(crate) owner: Option<String>,
    pub(crate) uses: Option<i32>,
    pub(crate) ips: Option<Vec<String>>,
    pub(crate) user_agent: Option<String>,
    pub(crate) created_by: Option<String>,
    pub(crate) notes: Option<String>,
}