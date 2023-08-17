use validators::prelude::*;

#[derive(Validator)]
#[validator(domain(ipv4(Allow), local(Allow), at_least_two_labels(Allow), port(NotAllow)))]
pub struct DomainWithoutPort(pub String);


pub fn is_valid_domain(domain: &String) -> bool {
    DomainWithoutPort::parse_string(domain).is_ok()
}