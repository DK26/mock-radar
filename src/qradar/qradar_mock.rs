use crate::permissions;

use super::reference_data::sets::ReferenceSet;
use std::collections::HashMap;

#[derive(Debug)]
pub(crate) struct QRadarMock {
    reference_sets: HashMap<String, ReferenceSet>,
}

impl QRadarMock {
    pub(crate) fn new() -> Self {
        QRadarMock {
            reference_sets: HashMap::new(),
        }
    }

    pub(crate) fn readonly_reference_sets(
        &self,
        _: permissions::AuthorizationToken,
    ) -> &HashMap<String, ReferenceSet> {
        &self.reference_sets
    }

    pub(crate) fn write_reference_sets(
        &mut self,
        _: permissions::AuthorizationToken,
    ) -> &mut HashMap<String, ReferenceSet> {
        &mut self.reference_sets
    }
}
