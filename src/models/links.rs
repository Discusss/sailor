use diesel;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use super::schema::links;
use super::schema::links::dsl::links as all_links;

#[derive(Serialize, Queryable)]
#[diesel(table_name = crate::schema::links)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Links {
    pub id: i32,
    pub domain: String,
    pub category: String,
    pub priority: i32,
    pub public_notes: String,

    pub submitted_by: String,
    pub submitted_at: std::time::SystemTime,
    pub submitted_ip: String,
    pub submitted_user_agent: String,
    pub submitted_reason: String,

    pub approved_by: String,
    pub approved_at: std::time::SystemTime,
    pub approved_key: String,

    pub notes: String,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "links"]
pub struct NewLink {
    pub domain: String,

    pub submitted_by: String,
    pub submitted_at: std::time::SystemTime,
    pub submitted_ip: String,
    pub submitted_user_agent: String,
    pub submitted_reason: String,
}

impl Links {

    pub fn all(connection: &PgConnection) -> Vec<Links> {
        all_links
            .order(links::id.desc())
            .load::<Links>(connection)
            .expect("Error loading links")
    }

    pub fn insert(link: NewLink, connection: &PgConnection) -> bool {
        diesel::insert_into(links::table)
            .values(&link)
            .execute(connection)
            .is_ok()
    }

}