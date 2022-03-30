pub struct PylaEntry<'a> {
  pub key: &'a [u8],
  pub value: &'a [u8],
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
