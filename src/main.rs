mod handlers;
mod permissions;
mod qradar;

use anyhow::Context;
use axum::Router;

use tower_http::trace::TraceLayer; // Middleware for high-level logging of requests/responses
use tracing::info; // Macros for logging and instrumenting functions
use tracing_subscriber::{EnvFilter, FmtSubscriber}; // Subscriber to format and filter trace data

use crate::permissions::REGISTERED_BASIC;
use crate::permissions::REGISTERED_PASSWORD;
use crate::permissions::REGISTERED_TOKEN;
use crate::permissions::REGISTERED_USERNAME;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize the tracing subscriber with an environment filter
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::new("debug")) // Default level is info, but can be overridden by RUST_LOG
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .context("setting default subscriber failed")?;

    let router = Router::new()
        .route("/", axum::routing::get(handlers::general::root))
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;

    info!("Serving QRadar mock on 127.0.0.1:3000");

    println!(
        r#"
        TOKEN = {REGISTERED_TOKEN}
        USERNAME = {REGISTERED_USERNAME}
        PASSWORD = {REGISTERED_PASSWORD}
        BASIC = {REGISTERED_BASIC}
    "#
    );

    axum::serve(listener, router).await?;

    Ok(())
}
