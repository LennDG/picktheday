use picktheday::{app, config::web_config};

#[tokio::main]
async fn main() {
    use axum::Router;
    use picktheday::fileserv::file_and_error_handler;
    use picktheday::plan_page;
    use tracing::info;

    // Setup tracing subscriber
    tracing_subscriber::fmt()
        .without_time()
        .with_target(false)
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

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    info!("{:<12} - {:?}\n", "LISTENING", listener.local_addr());
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
