use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "keys")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub key: String,
    pub created_at: DateTime,
    pub expires_at: Option<DateTime>,
    pub last_used_at: Option<DateTime>,
    pub owner: String,
    pub uses: i32,
    pub ips: Vec<String>,
    pub user_agent: String,
    pub created_by: String,
    pub notes: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::domains::Entity")]
    Domains,
}

impl Related<super::domains::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Domains.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
