pub mod links;
pub mod errors;
pub mod keys;

pub fn api() -> Vec<rocket::Route> {
    return routes![links::get_domain, keys::create_key, keys::get_key, keys::delete_key, keys::update_key];
}

pub fn catchers() -> Vec<rocket::Catcher> {
    return catchers![errors::not_found, errors::internal_error];
}