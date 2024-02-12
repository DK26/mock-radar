// Full permissions (Read & Write), can do anything
pub const REGISTERED_SEC_TOKEN: &str = "3472f695-9ea3-43f4-8762-55d259446c60";

// Can only access endpoints using the `GET` method
pub const REGISTERED_READONLY_SEC_TOKEN: &str = "d6391576-55d3-4c44-85d8-5665b0d2336f";

// Behaves same as if no SEC token was provider. It exists so we can formally address this behavior
pub const REGISTERED_EXPIRED_SEC_TOKEN: &str = "e3b189ab-3e3d-4ed8-a8e1-be4b75a6c3c0";

// This token exist for testing situations where the token has been authenticated, but it has no permissions
// to a specific resource that we are testing, and we would like to prompt the no-permissions response
pub const REGISTERED_MISSING_PERMISSIONS_SEC_TOKEN: &str = "a22b6757-57cb-4345-a812-e0a1e858cbb6";

// Full permissions (Read & Write), can do anything
pub const REGISTERED_USERNAME: &str = "admin";
pub const REGISTERED_PASSWORD: &str = "pass";
pub const REGISTERED_BASIC_TOKEN: &str = "YWRtaW46cGFzcw==";

#[derive(thiserror::Error, Debug)]
pub(crate) enum PermissionsError {
    #[error("failed to obtain write permission")]
    WritePermissionForbidden,

    #[error("failed to obtain read permission")]
    ReadPermissionForbidden,
}

/// Simply prevents direct initialization of the stateless `AuthorizationToken` type
#[allow(dead_code)]
#[derive(Debug)]
struct InitializationBlocker;

pub(crate) enum AuthenticationToken {
    Sec(String),
    Basic(String),
}

#[derive(Debug)]
pub(crate) struct AuthenticationProof {
    pub(crate) write_permission: Option<WritePermission>,
    pub(crate) read_permission: Option<ReadPermission>,
}

#[derive(Debug)]
pub(crate) struct WritePermission {
    #[allow(dead_code)]
    _blocker: InitializationBlocker,
}

impl TryFrom<AuthenticationProof> for WritePermission {
    type Error = PermissionsError;

    fn try_from(authorization_token: AuthenticationProof) -> Result<Self, Self::Error> {
        authorization_token
            .write_permission
            .ok_or(PermissionsError::WritePermissionForbidden)
    }
}

#[derive(Debug)]
pub(crate) struct ReadPermission {
    #[allow(dead_code)]
    _blocker: InitializationBlocker,
}

impl TryFrom<AuthenticationProof> for ReadPermission {
    type Error = PermissionsError;

    fn try_from(authorization_token: AuthenticationProof) -> Result<Self, Self::Error> {
        authorization_token
            .read_permission
            .ok_or(PermissionsError::ReadPermissionForbidden)
    }
}

impl AuthenticationProof {
    pub(crate) fn validate(authentication: AuthenticationToken) -> Option<Self> {
        match authentication {
            AuthenticationToken::Sec(token) if token.as_str() == REGISTERED_SEC_TOKEN => {
                Some(Self {
                    write_permission: Some(WritePermission {
                        _blocker: InitializationBlocker,
                    }),
                    read_permission: Some(ReadPermission {
                        _blocker: InitializationBlocker,
                    }),
                })
            }
            AuthenticationToken::Sec(token) if token.as_str() == REGISTERED_READONLY_SEC_TOKEN => {
                Some(Self {
                    write_permission: None,
                    read_permission: Some(ReadPermission {
                        _blocker: InitializationBlocker,
                    }),
                })
            }

            AuthenticationToken::Sec(token)
                if token.as_str() == REGISTERED_MISSING_PERMISSIONS_SEC_TOKEN =>
            {
                Some(Self {
                    write_permission: None,
                    read_permission: None,
                })
            }
            AuthenticationToken::Basic(token) if token.as_str() == REGISTERED_BASIC_TOKEN => {
                Some(Self {
                    write_permission: Some(WritePermission {
                        _blocker: InitializationBlocker,
                    }),
                    read_permission: Some(ReadPermission {
                        _blocker: InitializationBlocker,
                    }),
                })
            }

            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        permissions::REGISTERED_BASIC_TOKEN, REGISTERED_MISSING_PERMISSIONS_SEC_TOKEN,
        REGISTERED_READONLY_SEC_TOKEN,
    };

    use super::{AuthenticationProof, AuthenticationToken, REGISTERED_SEC_TOKEN};

    #[test]
    fn token_authentication_success() {
        let maybe_authorization_token = AuthenticationProof::validate(AuthenticationToken::Sec(
            REGISTERED_SEC_TOKEN.to_string(),
        ));

        assert!(maybe_authorization_token.is_some());
    }

    #[test]
    fn token_authentication_failure() {
        let maybe_authorization_token = AuthenticationProof::validate(AuthenticationToken::Sec(
            REGISTERED_BASIC_TOKEN.to_string(),
        ));

        assert!(maybe_authorization_token.is_none());
    }

    #[test]
    fn token_authentication_read_write_permissions_success() {
        let maybe_authorization_token = AuthenticationProof::validate(AuthenticationToken::Sec(
            REGISTERED_SEC_TOKEN.to_string(),
        ));

        let authorization_token = maybe_authorization_token.expect("unable to authenticate token");

        assert!(authorization_token.read_permission.is_some());
        assert!(authorization_token.write_permission.is_some());
    }

    #[test]
    fn token_authentication_read_only_permissions_success() {
        let maybe_authorization_token = AuthenticationProof::validate(AuthenticationToken::Sec(
            REGISTERED_READONLY_SEC_TOKEN.to_string(),
        ));

        let authorization_token = maybe_authorization_token.expect("unable to authenticate token");

        assert!(authorization_token.read_permission.is_some());
        assert!(authorization_token.write_permission.is_none());
    }

    #[test]
    fn token_authentication_missing_permissions_success() {
        let maybe_authorization_token = AuthenticationProof::validate(AuthenticationToken::Sec(
            REGISTERED_MISSING_PERMISSIONS_SEC_TOKEN.to_string(),
        ));

        let authorization_token = maybe_authorization_token.expect("unable to authenticate token");

        assert!(authorization_token.read_permission.is_none());
        assert!(authorization_token.write_permission.is_none());
    }

    #[test]
    fn basic_authentication_success() {
        let maybe_authorization_token = AuthenticationProof::validate(AuthenticationToken::Basic(
            REGISTERED_BASIC_TOKEN.to_string(),
        ));

        assert!(maybe_authorization_token.is_some());
    }

    #[test]
    fn basic_authentication_failure() {
        let maybe_authorization_token = AuthenticationProof::validate(AuthenticationToken::Basic(
            REGISTERED_SEC_TOKEN.to_string(),
        ));

        assert!(maybe_authorization_token.is_none());
    }
}
