pub use sea_orm_migration::prelude::*;

mod m20230816_182907_domains;
mod m20230816_184013_keys;
mod m20230817_155040_blacklist;
mod m20230917_150614_logs;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230917_150614_logs::Migration),
            Box::new(m20230817_155040_blacklist::Migration),
            Box::new(m20230816_184013_keys::Migration),
            Box::new(m20230816_182907_domains::Migration),
        ]
    }
}
