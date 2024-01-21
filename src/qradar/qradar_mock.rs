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
}
