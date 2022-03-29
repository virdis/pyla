use std::io;

use crate::ingestion::pyla_internals::PylaEntry;
use crate::ingestion::pyla_internals::PylaId;

pub trait PylaMap<'a> {

  fn read(&self, id: PylaId) -> Option<PylaEntry<'a>>;
  fn write(&self, key: &'a [u8], value: &'a [u8]) -> Result<(), io::Error>;
  
}

