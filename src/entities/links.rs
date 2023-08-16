use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "links")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_type = "Text")]
    pub domain: String,
    pub category: String,
    pub priority: i32,
    #[sea_orm(column_type = "Text")]
    pub public_notes: String,
    #[sea_orm(column_type = "Text")]
    pub submitted_by: String,
    pub submitted_at: Option<DateTime>,
    #[sea_orm(column_type = "Text", nullable)]
    pub submitted_ip: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub submitted_user_agent: Option<String>,
    #[sea_orm(column_type = "Text")]
    pub submitted_reason: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub approved_by: Option<String>,
    pub approved_at: Option<DateTime>,
    #[sea_orm(column_type = "Text")]
    pub approved_key: String,
    #[sea_orm(column_type = "Text")]
    pub notes: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::keys::Entity",
        from = "Column::ApprovedKey",
        to = "super::keys::Column::Key",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Keys,
}

impl Related<super::keys::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Keys.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
