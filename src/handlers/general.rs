use crate::permissions::REGISTERED_BASIC;
use crate::permissions::REGISTERED_PASSWORD;
use crate::permissions::REGISTERED_TOKEN;
use crate::permissions::REGISTERED_USERNAME;

#[tracing::instrument(level = "debug", ret)]
pub(crate) async fn root() -> String {
    format!(
        r#"
        TOKEN = {REGISTERED_TOKEN}
        USERNAME = {REGISTERED_USERNAME}
        PASSWORD = {REGISTERED_PASSWORD}
        BASIC = {REGISTERED_BASIC}
    "#
    )
}
