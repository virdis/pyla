use crossbeam_skiplist::SkipMap;
use crate::ingestion::pyla_map::PylaMap;

pub struct PylaInMemoryMap<'a> {
  inner: SkipMap<&'a [u8], &'a [u8]>,
}

impl<'a> PylaMap<'a> for PylaInMemoryMap<'a> {
    fn read(&self, id: super::PylaId) -> Option<super::PylaEntry<'a>> {
        todo!()
    }

    fn write(&self, key: &'a [u8], value: &'a [u8]) -> Result<(), std::io::Error> {
        todo!()
    }
}