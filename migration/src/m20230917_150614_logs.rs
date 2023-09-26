use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table(Log::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Log::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Log::Date)
                            .timestamp()
                            .not_null()
                            .default("now()"),
                    )
                    .col(ColumnDef::new(Log::Version).string().not_null().default("0.0.0"))
                    .col(ColumnDef::new(Log::Environment).string().not_null().default("production"))
                    .col(ColumnDef::new(Log::Level).string().not_null().default("info"))
                    .col(ColumnDef::new(Log::User).string().not_null().default("system"))
                    .col(ColumnDef::new(Log::LogEvent).string().not_null().default("unknown"))
                    .col(ColumnDef::new(Log::FilePath).string().not_null().default(""))
                    .col(ColumnDef::new(Log::Data).string().not_null().default(""))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Log::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Log {
    Table,
    Id,
    Date,
    Version,
    Environment,
    Level,
    User,
    LogEvent,
    FilePath,
    Data
}
