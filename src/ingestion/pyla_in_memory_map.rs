
use std::borrow::{Borrow, BorrowMut};

use crate::ingestion::PylaEntry;
use crossbeam_skiplist::{SkipMap};

use super::{PylaId, pyla_internals::NotFound};
pub struct PylaInMemoryMap {
    inner: SkipMap<PylaId, PylaEntry>, // make this Arc
}

impl Default for PylaInMemoryMap {
    fn default() -> Self {
        PylaInMemoryMap { inner: SkipMap::new() }
    }
}

impl PylaInMemoryMap {
    // this should happen on another thread
    pub fn get(&self, key: PylaId) -> Result<PylaEntry, NotFound> {
       if let Some(entry) = self
        .inner.get(&key) {
            return Ok(entry.value().to_owned())
        }
        return Err(NotFound::new())
        
    }

    pub fn set(&self, key: PylaId, value: PylaEntry)  {
        self.inner.insert(key, value);
    }
}
