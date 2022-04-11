use std::sync::atomic::AtomicUsize;

use crate::ingestion::PylaEntry;
use crossbeam_skiplist::{SkipMap};

use super::{PylaId, pyla_internals::NotFound};
pub struct PylaInMemoryMap {
    inner: SkipMap<PylaId, PylaEntry>, // make this Arc
    approximate_size: AtomicUsize,
}

impl Default for PylaInMemoryMap {
    fn default() -> Self {
        PylaInMemoryMap { inner: SkipMap::new(), approximate_size: AtomicUsize::new(0) }
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

    pub async fn async_get(&'static self, key: PylaId) -> Result<PylaEntry, NotFound> {
        tokio::spawn(async move {
            self.get(key)
        }).await.unwrap()
        
    }

    pub fn set(&self, key: PylaId, value: PylaEntry)  {
        self.inner.insert(key, value);
    }

    pub async fn async_set(&'static self, key: PylaId, value: PylaEntry) {
        tokio::spawn(async move {
            self.set(key, value)
        }).await.unwrap()
    }
}
