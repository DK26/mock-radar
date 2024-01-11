use std::collections::{HashMap, HashSet};

use crate::permissions;

#[derive(Hash, Eq, PartialEq, Debug)]
enum QRadarValue {
    AlphaNumeric(String),
    AlphaNumericIgnoreCase(String),
    Numeric(u64),
    Port(u16),
    IP(String),
}

struct QRadarMock {
    reference_sets: HashMap<String, HashSet<QRadarValue>>,
}

impl QRadarMock {
    pub(crate) fn new() -> Self {
        QRadarMock {
            reference_sets: HashMap::new(),
        }
    }
    pub(crate) fn add_to_reference_set(
        &mut self,
        _: permissions::AuthorizationToken,
        name: String,
        value: QRadarValue,
    ) {
        todo!()
        // self.reference_sets.insert(name, value);
    }
}

#[cfg(test)]
mod tests {}
