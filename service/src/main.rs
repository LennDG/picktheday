use config::web_config;
use fileserv::file_and_error_handler;

pub mod app;
pub mod config;
pub mod error;
pub mod fileserv;
pub mod htmx_helpers;
pub mod plan_page;
pub mod util_components;

#[tokio::main]
async fn main() {
    use axum::Router;
    use tracing::info;

    // Setup tracing subscriber
    tracing_subscriber::fmt()
        .without_time()
        .with_target(false)
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // Get config
    let config = &web_config();

    // Get the DB
    let mm = entity::db::ModelManager::new(config.DATABASE_URL.clone())
        .await
        .expect("Could not establish DB connection");

    // Run migrations
    mm.run_migrations().await.expect("Migrations failed!");

    // build our application with a route
    let app = Router::new()
        .merge(app::routes(mm.clone()))
        .merge(plan_page::routes(mm.clone()))
        .fallback(file_and_error_handler)
        .layer(tower_http::compression::CompressionLayer::new().zstd(true));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("{:<12} - {:?}\n", "LISTENING", listener.local_addr());
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
