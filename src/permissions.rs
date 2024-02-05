pub const REGISTERED_SEC_TOKEN: &str = "d6391576-55d3-4c44-85d8-5665b0d2336f";
pub const REGISTERED_USERNAME: &str = "admin";
pub const REGISTERED_PASSWORD: &str = "pass";
pub const REGISTERED_BASIC_TOKEN: &str = "YWRtaW46cGFzcw==";

/// Simply prevents direct initialization of the stateless `AuthorizationToken` type
#[allow(dead_code)]
#[derive(Debug)]
struct InitializationBlocker;

pub(crate) enum Authentication {
    SecToken(String),
    BasicToken(String),
}

#[derive(Debug)]
pub(crate) struct AuthorizationToken {
    #[allow(dead_code)]
    _blocker: InitializationBlocker,
}

impl AuthorizationToken {
    pub(crate) fn validate(authentication: Authentication) -> Option<Self> {
        match authentication {
            Authentication::SecToken(token) => {
                (token.as_str() == REGISTERED_SEC_TOKEN).then_some(Self {
                    _blocker: InitializationBlocker,
                })
            }
            Authentication::BasicToken(token) => (token.as_str() == REGISTERED_BASIC_TOKEN)
                .then_some(Self {
                    _blocker: InitializationBlocker,
                }),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::permissions::REGISTERED_BASIC_TOKEN;

    use super::{Authentication, AuthorizationToken, REGISTERED_SEC_TOKEN};

    #[test]
    fn token_authentication_success() {
        let maybe_authorization_token = AuthorizationToken::validate(Authentication::SecToken(
            REGISTERED_SEC_TOKEN.to_string(),
        ));

        assert!(maybe_authorization_token.is_some());
    }

    #[test]
    fn token_authentication_failure() {
        let maybe_authorization_token = AuthorizationToken::validate(Authentication::SecToken(
            REGISTERED_BASIC_TOKEN.to_string(),
        ));

        assert!(maybe_authorization_token.is_none());
    }

    #[test]
    fn basic_authentication_success() {
        let maybe_authorization_token = AuthorizationToken::validate(Authentication::BasicToken(
            REGISTERED_BASIC_TOKEN.to_string(),
        ));

        assert!(maybe_authorization_token.is_some());
    }

    #[test]
    fn basic_authentication_failure() {
        let maybe_authorization_token = AuthorizationToken::validate(Authentication::BasicToken(
            REGISTERED_SEC_TOKEN.to_string(),
        ));

        assert!(maybe_authorization_token.is_none());
    }
}
