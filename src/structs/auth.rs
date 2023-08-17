use rocket::Request;
use rocket::request::{FromRequest, Outcome};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use crate::entities::keys;
use crate::entities::prelude::Keys;

pub struct Auth {
    pub is_valid: bool,
    pub key: Option<String>,
    pub data: Option<keys::Model>,
    pub error_message: Option<String>,
}

pub async fn get_key(key: &String, db: &DatabaseConnection) -> Option<keys::Model> {

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
                key: None,
                data: None,
                error_message: Some("No Authorization header provided".to_string()),
            }),
        }.replace("Bearer ", "");


        let data = match get_key(&key, db).await {
            Some(key) => key,
            None => return Outcome::Success(Auth {
                is_valid: false,
                key: None,
                data: None,
                error_message: Some("Invalid key".to_string()),
            }),
        };

        if data.expires_at.unwrap() < chrono::Utc::now().naive_utc() {
            return Outcome::Success(Auth {
                is_valid: false,
                key: None,
                data: None,
                error_message: Some("Key expired, please contact support to renew your key".to_string()),
            });
        }

        return Outcome::Success(Auth {
            is_valid: true,
            key: Some(key),
            data: Some(data),
            error_message: None,
        });
    }
}