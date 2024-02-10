use crate::permissions::REGISTERED_BASIC_TOKEN;
use crate::permissions::REGISTERED_EXPIRED_SEC_TOKEN;
use crate::permissions::REGISTERED_PASSWORD;
use crate::permissions::REGISTERED_READONLY_SEC_TOKEN;
use crate::permissions::REGISTERED_SEC_TOKEN;
use crate::permissions::REGISTERED_USERNAME;

#[tracing::instrument(level = "debug", ret)]
pub(crate) async fn root() -> String {
    format!(
        r#"
        TOKEN = {REGISTERED_SEC_TOKEN}
        READ-ONLY TOKEN = {REGISTERED_READONLY_SEC_TOKEN}
        EXPIRED TOKEN = {REGISTERED_EXPIRED_SEC_TOKEN}
        USERNAME = {REGISTERED_USERNAME}
        PASSWORD = {REGISTERED_PASSWORD}
        BASIC = {REGISTERED_BASIC_TOKEN}
    "#
    )
}
