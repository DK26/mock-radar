mod reference_data;

use reference_data::sets::ReferenceSet;
use std::collections::HashMap;

#[derive(Debug)]
struct QRadarMock {
    reference_sets: HashMap<String, ReferenceSet>,
}

impl QRadarMock {
    pub(crate) fn new() -> Self {
        QRadarMock {
            reference_sets: HashMap::new(),
        }
    }
}
