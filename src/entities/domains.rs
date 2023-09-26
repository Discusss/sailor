use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "domains")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub domain: String,
    pub category: i32,
    pub severity: i32,
    pub public_notes: String,
    pub submitted_by: String,
    pub submitted_at: DateTime,
    pub submitted_ip: Option<String>,
    pub submitted_user_agent: Option<String>,
    pub submitted_reason: String,
    pub approved_by: Option<String>,
    pub approved_at: Option<DateTime>,
    pub approved_key: Option<String>,
    pub notes: String,
    pub times_consulted: i32,
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
