use std::collections::btree_map::Values;

use bytes::{Bytes, BytesMut, BufMut};

#[derive(Debug, Clone)]
pub struct PylaEntry {
  pub underlying: Bytes
}

impl PylaEntry {
  pub fn new(key: &[u8], value: &[u8]) -> PylaEntry {
    let key_size = key.len() as u32;
    let value_size = value.len() as u32;
    let total_size = u64::BITS + key_size + u64::BITS + value_size; // keysize:actual_key:valuesize:actual_value
    let mut buf =
      BytesMut::with_capacity(total_size as usize);
      // TODO - Endianess 

    buf.put_u32(key_size);
    buf.put_slice(key);
    buf.put_u32(value_size);
    buf.put_slice(value);
    PylaEntry { underlying: buf.freeze() }
  }    
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

pub struct NotFound();

impl NotFound {
    pub fn new() -> NotFound {
      NotFound()
    }
}