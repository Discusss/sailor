pub mod links;
pub mod errors;
pub mod keys;
pub mod blacklist;

pub fn api() -> Vec<rocket::Route> {
    return routes![
        links::get_domain,

        keys::create_key,
        keys::get_key,
        keys::delete_key,
        keys::update_key,

        blacklist::get_all_blacklist,
        blacklist::get_blacklist,
        blacklist::create_blacklist,
        blacklist::update_blacklist,
        blacklist::delete_blacklist,
    ];
}

pub fn catchers() -> Vec<rocket::Catcher> {
    return catchers![errors::not_found, errors::internal_error];
}