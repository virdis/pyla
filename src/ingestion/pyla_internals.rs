use crossbeam_skiplist::SkipMap;

pub struct PylaEntry<'a> {
  pub key: &'a [u8],
  pub value: &'a [u8],
}

pub struct PylaId(u64);

pub struct PylaInMemoryMap<'a> {
  inner: SkipMap<&'a [u8], &'a [u8]>,
}