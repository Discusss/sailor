use chrono::NaiveDateTime;
use rocket::{Request};
use rocket::request::{FromRequest, Outcome};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use crate::entities::blacklist;
use crate::entities::prelude::Blacklist;

pub struct RemoteAddress {
    pub ip: String,
    pub user_agent: Option<String>,
    pub is_blacklisted: bool,
    pub ban_data: Option<blacklist::Model>,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for RemoteAddress {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let ip = get_ip(request);
        let db = request.rocket().state::<DatabaseConnection>().unwrap();
        let user_agent = get_user_agent(request);

        let ban = match Blacklist::find()
            .filter(blacklist::Column::Ip.contains(ip.clone()))
            .one(db)
            .await
        {
            Ok(ban) => match ban {
                Some(ban) => ban,
                None => return Outcome::Success(RemoteAddress {
                    ip,
                    user_agent,
                    is_blacklisted: false,
                    ban_data: None
                }),
            }
            Err(_) => return Outcome::Success(RemoteAddress {
                ip,
                user_agent,
                is_blacklisted: false,
                ban_data: None
            }),
        };

        let expires = match ban.expires_at {
            Some(date) => date,
            None => return Outcome::Success(RemoteAddress {
                ip,
                user_agent,
                is_blacklisted: true,
                ban_data: Some(ban)
            })
        };

        let now = chrono::Utc::now().naive_utc();
        if now > expires {
            return Outcome::Success(RemoteAddress {
                ip,
                user_agent,
                is_blacklisted: false,
                ban_data: None
            });
        }

        Outcome::Success(RemoteAddress {
            ip,
            user_agent,
            is_blacklisted: true,
            ban_data: Some(ban)
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

fn get_user_agent(request: &Request) -> Option<String> {
    let headers = request.headers();
    let user_agent = headers.get_one("User-Agent");

    return match user_agent {
        Some(user_agent) => Some(user_agent.to_string()),
        None => None
    };
}