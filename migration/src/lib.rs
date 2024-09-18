pub use sea_orm_migration::prelude::*;

mod m20240918_104347_create_plans;
mod m20240918_110512_create_users;
mod m20240918_111732_create_dates;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240918_104347_create_plans::Migration),
            Box::new(m20240918_110512_create_users::Migration),
            Box::new(m20240918_111732_create_dates::Migration),
        ]
    }
}
