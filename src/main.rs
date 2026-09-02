use axum::Router;
use axum::routing::get;
use tracing::info;
use tracing::level_filters::LevelFilter;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_max_level(LevelFilter::INFO).init();

    let app = Router::new().route("/", get(root));

    let addr = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    info!(address = addr, "server started");
    axum::serve(listener, app).await.expect("failed to run server");
}

async fn root() -> &'static str {
    "Hello, World!"
}
