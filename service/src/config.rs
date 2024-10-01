use crate::error::Result;
use dotenvy::dotenv;
use std::{env, sync::OnceLock};

pub fn web_config() -> &'static WebConfig {
    static INSTANCE: OnceLock<WebConfig> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        WebConfig::load_from_env()
            .unwrap_or_else(|ex| panic!("FATAL - WHILE LOADING CONF - Cause: {ex:?}"))
    })
}

#[allow(non_snake_case)]
pub struct WebConfig {
    pub WEB_FOLDER: String,
    pub DATABASE_URL: String,
}

impl WebConfig {
    fn load_from_env() -> Result<WebConfig> {
        dotenv()?;

        Ok(WebConfig {
            WEB_FOLDER: env::var("SERVICE_WEB_FOLDER")?,
            DATABASE_URL: env::var("DATABASE_URL")?,
        })
    }
}
