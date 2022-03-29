pub struct PylaEntry<'a> {
  pub key: &'a [u8],
  pub value: &'a [u8],
}

pub struct PylaId(u64);