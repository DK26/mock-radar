use std::collections::{HashMap, HashSet};

#[derive(Hash, Eq, PartialEq, Debug)]
enum QRadarValue {
    Integer(u64),
    String(String),
}

struct QRadarMock {
    reference_set: HashSet<QRadarValue>,
    reference_map: HashMap<String, QRadarValue>,
}

impl QRadarMock {
    pub(crate) fn new() -> Self {
        QRadarMock {
            reference_set: HashSet::new(),
            reference_map: HashMap::new(),
        }
    }
    pub(crate) fn add_to_reference_set(
        &mut self,
        _: permissions::AuthorizationToken,
        value: QRadarValue,
    ) {
        self.reference_set
            .insert(QRadarValue::String("hello, world".into()));
    }
}

mod permissions {
    use crate::REGISTERED_TOKEN;

    struct InitializePreventer;

    pub(crate) struct AuthorizationToken {
        preventer: InitializePreventer,
    }

    impl AuthorizationToken {
        pub(crate) fn validate(token: &str) -> Option<Self> {
            (token == REGISTERED_TOKEN).then_some(Self {
                preventer: InitializePreventer,
            })
        }
    }
}

#[cfg(test)]
mod tests {}
