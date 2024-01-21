pub(crate) const REGISTERED_TOKEN: &str = "d6391576-55d3-4c44-85d8-5665b0d2336f";
pub(crate) const REGISTERED_USERNAME: &str = "admin";
pub(crate) const REGISTERED_PASSWORD: &str = "pass";
pub(crate) const REGISTERED_BASIC: &str = "YWRtaW46cGFzcw==";

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
