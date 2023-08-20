use sea_orm_migration::prelude::*;
use crate::m20230816_184013_keys::Keys;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table(Links::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Links::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Links::Domain).string().not_null())
                    .col(ColumnDef::new(Links::Category).integer().not_null().default(7))
                    .col(ColumnDef::new(Links::Priority).integer().not_null().default(0))
                    .col(ColumnDef::new(Links::PublicNotes).string().not_null().default(""))

                    .col(ColumnDef::new(Links::SubmittedBy).string().not_null())
                    .col(ColumnDef::new(Links::SubmittedAt).timestamp().not_null())
                    .col(ColumnDef::new(Links::SubmittedIp).string())
                    .col(ColumnDef::new(Links::SubmittedUserAgent).string())
                    .col(ColumnDef::new(Links::SubmittedReason).string().not_null())

                    .col(ColumnDef::new(Links::ApprovedBy).string())
                    .col(ColumnDef::new(Links::ApprovedAt).timestamp())
                    .col(ColumnDef::new(Links::ApprovedKey).string())

                    .col(ColumnDef::new(Links::Notes).string().not_null().default(""))
                    .col(ColumnDef::new(Links::TimesConsulted).integer().not_null().default(0))
                    .foreign_key(
                        ForeignKey::create()
                            .name("links_approved_key_fkey")
                            .from(Keys::Table, Keys::Key)
                            .to(Links::Table, Links::ApprovedKey)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .drop_table(Table::drop().table(Links::Table).to_owned())
            .await
    }
}
/**
    id serial primary key not null,
    domain text not null,
    category int not null default 7,
    priority int not null default 0,
    public_notes text not null default '',

    submitted_by text not null, -- the username of the user who submitted the link
    submitted_at timestamp default current_timestamp, -- the time the link was submitted
    submitted_ip text, -- the IP address of the user who submitted the link
    submitted_user_agent text, -- the user agent of the user who submitted the link
    submitted_reason text not null, -- the reason the user submitted the link (e.g. "Phishing link found on <server>")

    approved_by text, -- the username of the user who approved the link
    approved_at timestamp, -- the time the link was approved
    approved_key text, -- the key used to approve the link, references keys(key)

    notes text not null default '', -- the internal notes for the link
    times_consulted int not null default 0 -- the number of times the link has been consulted
*/
#[derive(DeriveIden)]
pub enum Links {
    Table,
    Id,
    Domain,
    Category,
    Priority,
    PublicNotes,

    SubmittedBy,
    SubmittedAt,
    SubmittedIp,
    SubmittedUserAgent,
    SubmittedReason,

    ApprovedBy,
    ApprovedAt,
    ApprovedKey,

    Notes,
    TimesConsulted,
}
