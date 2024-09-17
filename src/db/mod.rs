pub use self::error::{Error, Result};
use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    PgConnection,
};
use dotenvy::dotenv;
use std::env;

pub mod error;
pub mod models;
pub mod schema;
pub mod types;

//#[cfg(test)]
pub mod _dev_utils;

pub type Db = Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = PooledConnection<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct ModelManager {
    db: Db,
}

impl ModelManager {
    pub async fn new() -> Result<Self> {
        // TODO: Do so checking here?
        let db = get_connection_pool();

        Ok(ModelManager { db })
    }

    pub fn get_conn(self) -> DbConnection {
        // FIXME: don't unwrap
        self.db.get().unwrap()
    }
}

pub fn get_connection_pool() -> Db {
    let url = database_url_for_env();
    let manager = ConnectionManager::<PgConnection>::new(url);
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}

fn database_url_for_env() -> String {
    dotenv().ok();
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}
