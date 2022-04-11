use serde::{Deserialize};


#[derive(Deserialize)]
pub struct Settings {

}

#[derive(Deserialize)]
pub struct MapSettings {
  size_in_mb: i32
}

impl MapSettings {
  fn convert_mb_to_bytes(&self) -> i64 {
    (self.size_in_mb * 1024 * 1024).into()
  }
}
