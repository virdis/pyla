use crossbeam_skiplist::SkipMap;
use crate::ingestion::PylaEntry;

use super::PylaId;
pub struct PylaInMemoryMap<'a> {
  inner: SkipMap<PylaId, &'a [u8]>,
}

impl<'a> PylaInMemoryMap<'a> {
    pub fn get(&self, pyla_entry: PylaId) -> Option<PylaEntry<'a>> {
        match self.inner.get(&pyla_entry) {
          Some(ref entry) => todo!(), 
            None => None,
        }
    }
}