use serde::Deserialize;
use std::error::Error;
use std::fs;

const CONFIG_PATH: &'static str = "etc/heavysquid.toml";

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct TomlConfig {
    dir_baseline: String,
    dir_target: String,
}

impl TomlConfig {
    pub fn load() -> Result<TomlConfig, Box<dyn Error>> {
        let config_str = fs::read_to_string(CONFIG_PATH)?;
        let config_toml = toml::from_str(&config_str)?;

        Ok(config_toml)
    }

    pub fn get_dir_target(&self) -> &str {
        &self.dir_target
    }

    pub fn get_dir_baseline(&self) -> &str {
        &self.dir_baseline
    }
}
