
use std::borrow::Borrow;

use crate::ingestion::PylaEntry;
use crossbeam_skiplist::{SkipMap};

use super::{PylaId, pyla_internals::NotFound};
pub struct PylaInMemoryMap {
    inner: SkipMap<PylaId, PylaEntry>,
}

impl PylaInMemoryMap {
    pub fn get(&self, pyla_id: PylaId) -> Result<PylaEntry, NotFound> {
        match self.inner.get(&pyla_id) {
            Some(entry) => {
                Ok(entry.value().clone())
            },
            None => Err(NotFound::new()),
        }
    }
}
