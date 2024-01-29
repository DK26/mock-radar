pub const REGISTERED_TOKEN: &str = "d6391576-55d3-4c44-85d8-5665b0d2336f";
pub const REGISTERED_USERNAME: &str = "admin";
pub const REGISTERED_PASSWORD: &str = "pass";
pub const REGISTERED_BASIC: &str = "YWRtaW46cGFzcw==";

/// Simply prevents direct initialization of the stateless `AuthorizationToken` type
#[allow(dead_code)]
struct InitializationBlocker;

pub(crate) enum Authentication {
    Token(String),
    Basic(String),
}

pub(crate) struct AuthorizationToken {
    #[allow(dead_code)]
    _blocker: InitializationBlocker,
}

impl AuthorizationToken {
    pub(crate) fn validate(authentication: Authentication) -> Option<Self> {
        match authentication {
            Authentication::Token(token) => (token.as_str() == REGISTERED_TOKEN).then_some(Self {
                _blocker: InitializationBlocker,
            }),
            Authentication::Basic(token) => (token.as_str() == REGISTERED_BASIC).then_some(Self {
                _blocker: InitializationBlocker,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::permissions::REGISTERED_BASIC;

    use super::{Authentication, AuthorizationToken, REGISTERED_TOKEN};

    #[test]
    fn token_authentication_success() {
        let maybe_authorization_token =
            AuthorizationToken::validate(Authentication::Token(REGISTERED_TOKEN.to_string()));

        assert!(maybe_authorization_token.is_some());
    }

    #[test]
    fn token_authentication_failure() {
        let maybe_authorization_token =
            AuthorizationToken::validate(Authentication::Token(REGISTERED_BASIC.to_string()));

        assert!(maybe_authorization_token.is_none());
    }

    #[test]
    fn basic_authentication_success() {
        let maybe_authorization_token =
            AuthorizationToken::validate(Authentication::Basic(REGISTERED_BASIC.to_string()));

        assert!(maybe_authorization_token.is_some());
    }

    #[test]
    fn basic_authentication_failure() {
        let maybe_authorization_token =
            AuthorizationToken::validate(Authentication::Basic(REGISTERED_TOKEN.to_string()));

        assert!(maybe_authorization_token.is_none());
    }
}
