pub use self::error::{Error, Result};
use dotenvy::dotenv;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::env;
use std::time::Duration;

pub mod error;

//#[cfg(test)]
pub mod _dev_utils;

#[derive(Clone)]
pub struct ModelManager {
    db: DatabaseConnection,
}

impl ModelManager {
    pub async fn new() -> Result<Self> {
        // TODO: Do so checking here?
        let db = get_connection_pool().await?;

        Ok(ModelManager { db })
    }
}

pub async fn get_connection_pool() -> Result<DatabaseConnection> {
    let url = database_url_for_env();
    let mut opt = ConnectOptions::new(url);

    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8));

    Ok(Database::connect(opt).await?)
}

fn database_url_for_env() -> String {
    dotenv().ok();
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

// region:    --- Tests
#[cfg(test)]
mod tests {
    #![allow(unused)]
    use super::*;
    use anyhow::Result;
    use entity::{plans, types::PlanName};
    use sea_orm::{EntityTrait, Set};

    #[tokio::test]
    async fn test_create_plan_ok() -> Result<()> {
        let db = _dev_utils::init_test().await.db;

        let new_plan = plans::ActiveModel {
            name: Set(PlanName::new("Test").unwrap()),
            ..Default::default()
        };

        let result = plans::Entity::insert(new_plan).exec(&db).await?;
        Ok(())
    }
}
// endregion: --- Tests
