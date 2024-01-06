use axum::Router;

const REGISTERED_TOKEN: &str = "aabbcccdd";
const REGISTERED_USERNAME: &str = "admin";
const REGISTERED_PASSWORD: &str = "password";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let router = Router::new().route("/", axum::routing::get(root));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;

    axum::serve(listener, router).await?;

    Ok(())
}

async fn root() -> String {
    format!(
        r#"
        TOKEN = {REGISTERED_TOKEN}
        USERNAME = {REGISTERED_USERNAME}
        PASSWORD = {REGISTERED_PASSWORD}
    "#
    )
}
