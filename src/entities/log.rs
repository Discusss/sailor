use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "log")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub date: DateTime,
    pub version: String,
    pub environment: String,
    pub level: String,
    pub user: String,
    pub log_event: String,
    pub file_path: String,
    pub data: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
