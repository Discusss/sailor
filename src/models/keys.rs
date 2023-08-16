use diesel;
use diesel::prelude::*;

#[derive(Serialize, Queryable)]
pub struct Keys {
    pub key: String,
    pub created_at: std::time::SystemTime,
    pub expires_at: std::time::SystemTime,
    pub last_used_at: std::time::SystemTime,

    pub owner: String,
    pub uses: i32,

    pub ips: Vec<String>,
    pub user_agents: String,

    pub created_by: String,
    pub notes: String,
}