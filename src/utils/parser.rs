use sea_orm::DatabaseConnection;
use validators::prelude::*;
use crate::security::sql::check_and_ban;

#[derive(Validator)]
#[validator(domain(ipv4(Allow), local(Allow), at_least_two_labels(Allow), port(NotAllow)))]
pub struct DomainWithoutPort(pub String);

pub async fn is_valid_domain(domain: &String, db: &DatabaseConnection, ip: &String) -> bool {
    let domain = domain.replace("http://", "").replace("https://", "");
    let sql_checks = check_and_ban(&domain, db, ip).await;

    DomainWithoutPort::parse_string(domain).is_ok() && !sql_checks
}