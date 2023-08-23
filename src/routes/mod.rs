pub mod domains;
pub mod errors;
pub mod keys;
pub mod blacklist;
pub mod stats;

pub fn api() -> Vec<rocket::Route> {
    return routes![

        keys::create_key,
        keys::get_key,
        keys::delete_key,
        keys::update_key,

        blacklist::get_all_blacklist,
        blacklist::get_blacklist,
        blacklist::check_blacklist,
        blacklist::create_blacklist,
        blacklist::update_blacklist,
        blacklist::delete_blacklist,

        domains::get_domain,
        domains::create_domain,
        domains::update_domain,
        domains::delete_domain,
    ];
}

pub fn catchers() -> Vec<rocket::Catcher> {
    return catchers![
        errors::not_found,
        errors::internal_error,
        errors::forbidden,
        errors::unauthorized,
    ];
}