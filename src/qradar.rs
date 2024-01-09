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
    pub(crate) fn add_to_reference_set(&mut self, _: permissions::Modifier, value: QRadarValue) {
        self.reference_set
            .insert(QRadarValue::String("hello, world".into()));
    }
}

// TODO: Create Roles which implement Into<Modifier> & Into<Reader>. Or, Into<ReferenceSetModifier> & Into<ReferenceSetReader> etc.
mod permissions {

    struct InitializePreventer;

    pub(crate) struct Modifier {
        preventer: InitializePreventer,
    }

    impl Modifier {
        pub(crate) fn validate(user_id: u64) -> Option<Self> {
            None
        }
    }
}

#[cfg(test)]
mod tests {}
