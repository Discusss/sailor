use rocket::{Request};
use rocket::request::{FromRequest, Outcome};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use crate::entities::blacklist;
use crate::entities::prelude::Blacklist;

pub struct RemoteAddress {
    pub ip: String,
    pub is_blacklisted: bool
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for RemoteAddress {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let ip = get_ip(request);
        let db = request.rocket().state::<DatabaseConnection>().unwrap();

        match Blacklist::find()
            .filter(blacklist::Column::Ip.contains(ip.clone()))
            .one(db)
            .await
        {
            Ok(ban) => match ban {
                Some(_) => (),
                None => return Outcome::Success(RemoteAddress {
                    ip,
                    is_blacklisted: false
                }),
            }
            Err(_) => return Outcome::Success(RemoteAddress {
                ip,
                is_blacklisted: false
            }),
        };

        Outcome::Success(RemoteAddress {
            ip,
            is_blacklisted: true
        })
    }
}

pub fn get_ip(request: &Request) -> String {
    let ip = request.client_ip().unwrap().to_string(); // probably 127.0.0.1
    let headers = request.headers();
    let forwarded_for = headers.get_one("X-Forwarded-For");

    return match forwarded_for {
        Some(forwarded_for) => {
            let new_ip = match forwarded_for.parse::<std::net::IpAddr>() {
                Ok(new_ip) => new_ip.to_string(),
                Err(_) => return ip
            };
            new_ip
        }
        None => ip
    };
}