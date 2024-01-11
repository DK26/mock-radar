use crate::{REGISTERED_BASIC, REGISTERED_TOKEN};

struct InitializePreventer;

pub(crate) enum Authentication {
    Token(String),
    Basic(String),
}

pub(crate) struct AuthorizationToken {
    preventer: InitializePreventer,
}

impl AuthorizationToken {
    pub(crate) fn validate(authentication: Authentication) -> Option<Self> {
        match authentication {
            Authentication::Token(token) => (token.as_str() == REGISTERED_TOKEN).then_some(Self {
                preventer: InitializePreventer,
            }),
            Authentication::Basic(token) => (token.as_str() == REGISTERED_BASIC).then_some(Self {
                preventer: InitializePreventer,
            }),
        }
    }
}

#[cfg(test)]
mod tests {}
