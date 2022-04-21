use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

use crate::{
    configuration::{get_configuration, Settings},
    ingestion::{PylaEntry, PylaMapState},
};
use arc_swap::{ArcSwap};
use crossbeam_skiplist::SkipMap;
use super::{pyla_internals::NotFound, PylaId};

type InnerMap = ArcSwap<SkipMap<PylaId, PylaEntry>>;
pub struct PylaInMemoryMap {
    inner: InnerMap,
    approximate_size: AtomicUsize,
    config: Settings,
}

impl Default for PylaInMemoryMap {
    fn default() -> Self {
        PylaInMemoryMap {
            inner: ArcSwap::new(Arc::new(SkipMap::default())),
            approximate_size: AtomicUsize::new(0),
            config: get_configuration().expect("Failed to load configuration"),
        }
    }
}

impl PylaInMemoryMap {
    // this should happen on another thread
    pub fn get(&self, key: PylaId) -> Result<PylaEntry, NotFound> {

        if let Some(entry) = self.inner.load().get(&key) {
            return Ok(entry.value().to_owned());
        }
        return Err(NotFound::new());
    }

    pub async fn async_get(&'static self, key: PylaId) -> Result<PylaEntry, NotFound> {
        tokio::spawn(async move { self.get(key) }).await.unwrap()
    }

    pub fn set(&self, key: PylaId, value: PylaEntry) {
        self.inner.load().insert(key, value);
    }

    pub async fn async_set(&'static self, key: PylaId, value: PylaEntry) {
        let set_result = tokio::spawn(async move {
            let value_size = value.get_size();
            self.set(key, value);
            self.check_entry_size(value_size)
        })
        .await;
        tokio::spawn(async move {
         /*    if let Some(range) = self.extract_range(set_result) {
                range.map(|_| print!("")); // Build Block and delete elements from map
            } */
        });
        todo!()
    }

    fn extract_range<T>(&self, state_result: Result<PylaMapState, T>) {
        if let Ok(pyla_state) = state_result {
            if let PylaMapState::WriteToDisk(swapped_map) = pyla_state {
                
            }
        }
    }

    fn check_entry_size(&self, value_size: usize) -> PylaMapState {
        let result: PylaMapState;

        result = loop {
            let current_size = self.approximate_size.load(Ordering::SeqCst);
            let new_size = current_size + value_size;
            if new_size < self.config.map_settings.convert_mb_to_bytes() {
                match self.approximate_size.compare_exchange(
                    current_size,
                    new_size,
                    Ordering::SeqCst,
                    Ordering::SeqCst,
                ) {
                    Ok(_) => break PylaMapState::WriteToMap,
                    Err(x) => continue,
                }
            } else {
                let previous_map = self
                    .inner
                    .compare_and_swap(self.inner.load(), Arc::new(SkipMap::default()));
                self.approximate_size.store(0, Ordering::SeqCst);
                break PylaMapState::WriteToDisk(previous_map);
            }
        };
        return result;
    }
}
