use rocket::{Request};
use rocket::request::{FromRequest, Outcome};

pub struct RemoteAddress(pub(crate) String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for RemoteAddress {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let ip = request.client_ip().unwrap().to_string(); // probably 127.0.0.1
        let headers = request.headers();
        let forwarded_for = headers.get_one("X-Forwarded-For");

        return match forwarded_for {
            Some(forwarded_for) => {
                let new_ip = match forwarded_for.parse::<std::net::IpAddr>() {
                    Ok(new_ip) => new_ip.to_string(),
                    Err(_) => return Outcome::Success(RemoteAddress(ip))
                };
                Outcome::Success(RemoteAddress(new_ip))
            }
            None => Outcome::Success(RemoteAddress(ip))
        };
    }
}