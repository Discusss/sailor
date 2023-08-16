pub use sea_orm_migration::prelude::*;

mod m20230816_182907_links;
mod m20230816_184013_keys;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230816_182907_links::Migration),
            Box::new(m20230816_184013_keys::Migration),
        ]
    }
}
