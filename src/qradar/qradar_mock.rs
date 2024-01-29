use crate::permissions;

use super::reference_data::sets::ReferenceSet;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct QRadarMock {
    reference_sets: HashMap<String, ReferenceSet>,
}

impl QRadarMock {
    pub(crate) fn new() -> Self {
        QRadarMock {
            reference_sets: HashMap::new(),
        }
    }

    pub(crate) fn reference_sets_readonly_access(
        &self,
        _: permissions::AuthorizationToken,
    ) -> &HashMap<String, ReferenceSet> {
        &self.reference_sets
    }

    pub(crate) fn reference_sets_write_access(
        &mut self,
        _: permissions::AuthorizationToken,
    ) -> &mut HashMap<String, ReferenceSet> {
        &mut self.reference_sets
    }
}
