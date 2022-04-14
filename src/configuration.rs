use config::Config;
use serde::{Deserialize};


#[derive(Deserialize, Debug)]
pub struct Settings {
  pub map_settings: MapSettings
}

/// MapSettings represents the config for in memory map ie: PylaInMemoryMap.
/// size_in_mb represents the approximate size after which we write data
/// from InMemoryMapt to Disk.
/// Default value for this config = 2 Gb. Check base.yml
#[derive(Deserialize, Debug)]
pub struct MapSettings {
  size_in_mb: i32
}

impl MapSettings {
  
  pub fn convert_mb_to_bytes(&self) -> usize {
    (self.size_in_mb * 1024 * 1024) as usize
  }
}

/// Read the configuration present in the configuration folder.
/// We migh introduce concept of environments.
pub fn get_configuration() -> Result<Settings, config::ConfigError> {
  let mut builder = Config::builder();
  let base_path = std::env::current_dir()
    .expect("Failed to determine current directory");
  builder = builder.add_source(config::File::from(base_path
    .join("base")).required(true));
  
  builder.build().map(|cfg| cfg.try_deserialize().unwrap())
  
}