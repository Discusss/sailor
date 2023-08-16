use url::{Url, ParseError, Host};

pub fn is_valid_domain(domain: &String) -> bool {

    let domain = match Url::parse(domain) {
        Ok(domain) => domain,
        Err(_) => return false,
    };

    return false; //TODO: fix this
    //Host::Domain(domain.domain().unwrap()).is_some()

}