use std::sync::atomic::{AtomicUsize, Ordering};

use crate::{ingestion::PylaEntry, configuration::{Settings, get_configuration}};
use crossbeam_skiplist::{SkipMap};
use std::ops::Bound::{Unbounded, Included};

use super::{PylaId, pyla_internals::NotFound};
pub struct PylaInMemoryMap {
    inner: SkipMap<PylaId, PylaEntry>,
    approximate_size: AtomicUsize,
    config: Settings
}

impl Default for PylaInMemoryMap {
    fn default() -> Self {
        PylaInMemoryMap { inner: SkipMap::new(), 
            approximate_size: AtomicUsize::new(0),
            config: get_configuration().expect("Failed to load configuration")
        }
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
        let set_result = tokio::spawn(async move {
            let value_size = value.get_size();
            self.set(key, value);
            self.check_entry_size(value_size)
        }).await;
        if let Ok(pyla_state) = set_result {
            if let PylaMapState::WriteToDisk(optional_first_key, optional_last_key) = pyla_state {
                if let Some(first_key) = optional_first_key {
                    if let Some(last_key) = optional_last_key {
                        let first_id = PylaId::new(first_key);
                        let last_id = PylaId::new(last_key);
                        self.inner.range(( Included(first_id), Included(last_id)));
                    }
                }
            }       
        }
        todo!()
    }

    fn check_entry_size(&self, value_size: usize) -> PylaMapState {
        let result: PylaMapState;
        
        result = loop {
            let current_size = self.approximate_size.load(Ordering::SeqCst);
            let new_size = current_size + value_size;
            if new_size < self.config.map_settings.convert_mb_to_bytes() {
                match self.approximate_size.compare_exchange(current_size,
                     new_size, Ordering::SeqCst, Ordering::SeqCst) {
                         Ok(_) => break PylaMapState::WriteToMap,
                         Err(x) => continue,
                     }
            } else {
                // s.range((Unbounded, Included(&90)))
                let smallest_key: Option<u64> = self.inner.front().map(|entry| entry.key().into());
                let largest_key: Option<u64> = self
                    .inner.back().map(|entry| entry.key().into());
                break PylaMapState::WriteToDisk(smallest_key, largest_key);
            }
        };
        return result
    }
}


#[derive(Debug)]
enum PylaMapState {
    WriteToDisk(Option<u64>,Option<u64>),
    WriteToMap,   
}