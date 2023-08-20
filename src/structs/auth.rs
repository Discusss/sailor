use std::process::exit;
use rocket::Request;
use rocket::request::{FromRequest, Outcome};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use validators::prelude::*;
use crate::entities::keys;
use crate::entities::prelude::Keys;
use uuid::Uuid;
use crate::structs::ip::{get_ip};

pub struct Auth {
    pub is_valid: bool,
    pub is_master: bool,
    pub key: Option<String>,
    pub data: Option<keys::Model>,
    pub error_message: Option<String>,
}

pub async fn get_key(key: &String, db: &DatabaseConnection) -> Option<keys::Model> {

    let key = key.replace("Bearer ", "");
    if !is_valid_key(&key) {
        return None;
    }

    let k: keys::Model = match Keys::find()
        .filter(keys::Column::Key.eq(key))
        .one(db)
        .await
    {
        Ok(info) => match info {
            Some(info) => info,
            None => return None,
        },
        Err(_) => return None,
    };

    return Some(k);
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Auth {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let db = request.rocket().state::<DatabaseConnection>().unwrap();

        let key = match request.headers().get_one("Authorization") {
            Some(key) => key,
            None => return Outcome::Success(Auth {
                is_valid: false,
                is_master: false,
                key: None,
                data: None,
                error_message: Some("No Authorization header provided".to_string()),
            }),
        }.replace("Bearer ", "");

        if !is_valid_key(&key) {
            return Outcome::Success(Auth {
                is_valid: false,
                is_master: false,
                key: None,
                data: None,
                error_message: Some("Invalid key".to_string()),
            });
        }


        let data = match get_key(&key, db).await {
            Some(key) => key,
            None => return Outcome::Success(Auth {
                is_valid: false,
                is_master: false,
                key: None,
                data: None,
                error_message: Some("Invalid key".to_string()),
            }),
        };

        if data.expires_at.is_some() && data.expires_at.unwrap() < chrono::Utc::now().naive_utc() {
            return Outcome::Success(Auth {
                is_valid: false,
                is_master: false,
                key: None,
                data: None,
                error_message: Some("Key expired, please contact support to renew your key".to_string()),
            });
        }

        let master_key = std::env::var("MASTER_KEY").expect("MASTER_KEY must be set");
        if data.key != master_key {
            let mut key: keys::ActiveModel = data.into();

            let ip = get_ip(request);
            let user_agent = request.headers().get_one("User-Agent").unwrap_or("Unknown").to_string();

            key.last_used_at = Set(Some(chrono::Utc::now().naive_utc()));
            key.uses = Set(key.uses.unwrap() + 1);

            let mut ips = key.ips.clone().unwrap();
            if !ips.contains(&ip) {
                ips.push(ip.to_string());
                key.ips = Set(ips);
            }

            if user_agent != "Unknown" && user_agent != "" {
                key.user_agent = Set(user_agent);
            }

            return match key.update(db).await {
                Ok(new_key) => {
                    Outcome::Success(Auth {
                        is_valid: true,
                        is_master: false,
                        key: Some(new_key.key.clone()),
                        data: Some(new_key),
                        error_message: None,
                    })
                }
                Err(err) => {
                    error!("Error updating key: {}", err);
                    Outcome::Success(Auth {
                        is_valid: false,
                        is_master: false,
                        key: None,
                        data: None,
                        error_message: Some("Error updating key".to_string()),
                    })
                },
            };
        }

        return Outcome::Success(Auth {
            is_valid: true,
            is_master: true,
            key: Some(key),
            data: Some(data),
            error_message: None,
        });
    }
}

#[derive(Validator)]
#[validator(uuid(case(Any), separator(NotAllow)))]
pub struct ApiKey(pub u128);
impl ApiKey {
    pub fn generate() -> String {
        Uuid::new_v4().to_string().replace("-", "")
    }
}

pub fn is_valid_key(key: &String) -> bool {
    let key = key.replace("Bearer ", "");
    ApiKey::parse_string(&key).is_ok()
}

pub async fn validate_master_key(db: &DatabaseConnection) {
    let master_key = std::env::var("MASTER_KEY").expect("MASTER_KEY must be set");

    if !is_valid_key(&master_key) {
        error!("Invalid master key: key must be a valid UUID, received '{}'\nSuggested key: {}", master_key, ApiKey::generate());
        exit(1);
    }

    match Keys::find()
        .filter(keys::Column::Key.eq(master_key.clone()))
        .one(db)
        .await
    {
        Ok(info) => match info {
            Some(info) => info,
            None => {

                let now = chrono::Utc::now().naive_utc();
                let new_key = keys::ActiveModel {
                    key: Set(master_key.clone()),
                    created_at: Set(now),
                    expires_at: Set(None),
                    last_used_at: Set(None),
                    owner: Set("system".to_string()),
                    uses: Set(0),
                    ips: Set(vec![]),
                    user_agent: Set("system".to_string()),
                    created_by: Set("system".to_string()),
                    notes: Set("Master key".to_string()),
                };

                let new_key = match new_key.insert(db).await {
                    Ok(key) => key,
                    Err(err) => {
                        error!("Error creating master key: {}", err);
                        exit(1);
                    },
                };

                info!("Created master key: {}", new_key.key);
                new_key
            },
        },
        Err(_) => {
            error!("Error creating master key");
            exit(1);
        },
    };
}

pub fn obfuscate_key(key: String) -> String {
    let mut new_key = String::new();
    for (i, c) in key.chars().enumerate() {
        if i < key.len() - 4 {
            new_key.push('*');
        } else {
            new_key.push(c);
        }
    }
    return new_key;
}