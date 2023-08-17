use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "blacklist")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub ip: String,
    pub reason: String,
    pub expires_at: DateTime,
    pub created_at: DateTime,
    pub created_by: String,
    pub notes: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
