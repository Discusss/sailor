use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Blacklist::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Blacklist::Ip)
                            .string()
                            .not_null()
                            .unique_key()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Blacklist::Reason).string().not_null().default(""))
                    .col(ColumnDef::new(Blacklist::ExpiresAt).timestamp().not_null())
                    .col(ColumnDef::new(Blacklist::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Blacklist::CreatedBy).string().not_null().default("system"))
                    .col(ColumnDef::new(Blacklist::Notes).string().not_null().default(""))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .drop_table(Table::drop().table(Blacklist::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Blacklist {
    Table,
    Ip,
    Reason,
    ExpiresAt,
    CreatedAt,
    CreatedBy,
    Notes,
}
