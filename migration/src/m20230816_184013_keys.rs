use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table(Keys::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Keys::Key)
                            .string()
                            .not_null()
                            .unique_key()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Keys::CreatedAt).timestamp().default("current_timestamp"))
                    .col(ColumnDef::new(Keys::ExpiresAt).timestamp().default("current_timestamp + interval '1 year'"))
                    .col(ColumnDef::new(Keys::LastUsedAt).timestamp().default("current_timestamp"))

                    .col(ColumnDef::new(Keys::Owner).string().not_null().default("internal"))
                    .col(ColumnDef::new(Keys::Uses).integer().not_null().default(0))

                    .col(ColumnDef::new(Keys::Ips).array(ColumnType::Text).not_null().default("ARRAY[]::text[]"))
                    .col(ColumnDef::new(Keys::UserAgent).string().not_null().default("unknown"))

                    .col(ColumnDef::new(Keys::CreatedBy).string().not_null().default("system"))
                    .col(ColumnDef::new(Keys::Notes).string().not_null().default(""))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .drop_table(Table::drop().table(Keys::Table).to_owned())
            .await
    }
}

/**
    key text primary key unique not null, -- the key itself

    created_at timestamp not null default current_timestamp, -- when the key was created
    expires_at timestamp not null default current_timestamp + interval '1 year', -- when the key expires, default 1 year
    last_used_at timestamp not null default current_timestamp, -- when the key was last used

    owner text not null default 'internal', -- username of the owner of the key
    uses integer not null default 0, -- how many times the key has been used

    ips text[] not null default '{}', -- which IPs have used the key
    user_agent text not null default 'unknown', -- which user agents have used the key (last one)

    created_by text not null default 'system', -- who created the key
    notes text not null default '' -- internal notes about the key
*/
#[derive(DeriveIden)]
pub enum Keys {
    Table,
    Key,

    CreatedAt,
    ExpiresAt,
    LastUsedAt,

    Owner,
    Uses,

    Ips,
    UserAgent,

    CreatedBy,
    Notes,
}
