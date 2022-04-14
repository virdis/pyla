use std::{mem, ascii::AsciiExt, str::FromStr};

use bytes::{Bytes, BytesMut, BufMut};

const SIZE_OF_U32_IN_BYTES: usize = mem::size_of::<i32>();
const PYLA_KEY_ERROR: &str = "Key cannnot be empty";
const PYLA_VALUE_ERROR: &str = "Value cannot be emtpy";

#[derive(Debug, Clone)]
pub struct PylaEntry{
  pub underlying: Bytes // change visibility
}

impl PylaEntry {
  pub fn new(key: &[u8], value: &[u8]) -> Result<PylaEntry, PylaEntryError> {
    if key.is_empty() {
      return Err(PylaEntryError::KeyError(String::from(PYLA_KEY_ERROR)))
    }
     if value.is_empty() {
      return Err(PylaEntryError::KeyError(String::from(PYLA_VALUE_ERROR)))
    }
    
    let key_size = key.len() as u32;
    let value_size = value.len() as u32;
    let total_size: usize = SIZE_OF_U32_IN_BYTES + (key_size as usize) + SIZE_OF_U32_IN_BYTES + (value_size as usize);
    let mut buf = BytesMut::with_capacity(total_size);
      // TODO - Endianess 
    buf.put_u32(key_size);
    buf.put_u32(value_size);
    buf.put_slice(key);
    buf.put_slice(value);
    Ok(PylaEntry { underlying: buf.freeze() })
  } 
  
  pub fn get_size(&self) -> usize {
    self.underlying.len()
  }
}


#[derive(Debug)]
pub enum PylaEntryError {
  KeyError(String),
  ValueError(String),
}


/// PylaId represents a wrapper (new-type) around `u64` type.
///
/// We need a fixed length `id` that can be used across the project.
/// Since the incoming keys `&[u8]` have a variable length,
/// to get around it, we hash the key to generate a fixed length `id`.
/// 
/// We are using [xxHash] https://crates.io/crates/xxhash-rust. 
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct PylaId(u64);

impl PylaId {
  pub fn new(id: u64) -> PylaId {
    PylaId(id)
  }
}

impl From<&PylaId> for u64 {
    fn from(input: &PylaId) -> Self {
        input.0
    }
}
pub struct NotFound();

impl NotFound {
    pub fn new() -> NotFound {
      NotFound()
    }
}
