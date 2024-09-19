pub use self::error::{Error, Result};
use dotenvy::dotenv;
use sea_orm::{entity::*, ConnectOptions, Database, DatabaseConnection};
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

    pub async fn new_test() -> Result<Self> {
        // TODO: Do so checking here?
        let db = get_test_connection().await?;

        Ok(ModelManager { db })
    }

    pub fn db(&self) -> &DatabaseConnection {
        &self.db
    }
}

pub async fn get_test_connection() -> Result<DatabaseConnection> {
    let url = database_url_for_env();
    let mut opt = ConnectOptions::new(url);

    opt.max_connections(1)
        .min_connections(1)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8));

    Ok(Database::connect(opt).await?)
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
    use entity::{
        dates, plans,
        types::{PlanName, UserName},
        users,
    };
    use sea_orm::{EntityTrait, IntoActiveModel, Set};

    #[tokio::test]
    async fn test_create_plan_ok() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let db = mm.db();

        let new_plan = plans::NewPlan::new(PlanName::new("test_create_plan_ok").unwrap(), None)
            .into_active_model()
            .insert(db)
            .await?;

        // -- Check
        assert_eq!(new_plan.name.to_string(), "test_create_plan_ok".to_string());

        // -- Cleanup
        new_plan.delete(db).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_create_user_ok() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let db = mm.db();

        let new_plan = plans::NewPlan::new(PlanName::new("test_create_user_ok").unwrap(), None)
            .into_active_model()
            .insert(db)
            .await?;

        let new_user =
            users::NewUser::new(UserName::new("test_create_user_ok").unwrap(), new_plan.id)
                .into_active_model()
                .insert(db)
                .await?;

        // -- Check
        assert_eq!(new_user.name.to_string(), "test_create_user_ok".to_string());

        // -- Cleanup
        new_plan.delete(db).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_create_date_ok() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let db = mm.db();

        let new_plan = plans::NewPlan::new(PlanName::new("test_create_date_ok").unwrap(), None)
            .into_active_model()
            .insert(db)
            .await?;

        let new_user =
            users::NewUser::new(UserName::new("test_create_date_ok").unwrap(), new_plan.id)
                .into_active_model()
                .insert(db)
                .await?;

        let new_date = dates::NewDate::new(time::OffsetDateTime::now_utc().date(), new_user.id)
            .into_active_model()
            .insert(db)
            .await?;

        // -- Check
        assert_eq!(new_user.name.to_string(), "test_create_date_ok".to_string());
        let dates = new_user.find_related(dates::Entity).all(db).await?;
        assert_eq!(dates.len(), 1);

        // -- Cleanup
        new_plan.delete(db).await?;
        Ok(())
    }
}
// endregion: --- Tests
