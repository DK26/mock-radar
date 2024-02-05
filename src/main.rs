use anyhow::Context;

use mock_radar::REGISTERED_BASIC_TOKEN;
use mock_radar::REGISTERED_PASSWORD;
use mock_radar::REGISTERED_SEC_TOKEN;
use mock_radar::REGISTERED_USERNAME;

use tower_http::trace::TraceLayer; // Middleware for high-level logging of requests/responses
use tracing::info; // Macros for logging and instrumenting functions
use tracing_subscriber::{EnvFilter, FmtSubscriber};

use mock_radar::SharedQRadarMock; // Subscriber to format and filter trace data

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize the tracing subscriber with an environment filter
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::new("debug")) // Default level is debug, but can be overridden by RUST_LOG
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .context("setting default subscriber failed")?;

    let shared_state = SharedQRadarMock::default();

    let router = mock_radar::create_routes()
        .layer(TraceLayer::new_for_http())
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;

    info!("Serving QRadar mock on 127.0.0.1:3000");

    println!(
        r#"
        TOKEN = {REGISTERED_SEC_TOKEN}
        USERNAME = {REGISTERED_USERNAME}
        PASSWORD = {REGISTERED_PASSWORD}
        BASIC = {REGISTERED_BASIC_TOKEN}
    "#
    );

    axum::serve(listener, router).await?;

    Ok(())
}
