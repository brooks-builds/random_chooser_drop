use std::fs::File;
use std::io::Read;

use eyre::Result;

use self::config_struct::Config;
pub mod config_struct;

pub fn load_config(path: String) -> Result<Config> {
    let mut config_file = File::open(path)?;
    let mut config_json = String::new();
    config_file.read_to_string(&mut config_json)?;
    let config: Config = serde_json::from_str(&config_json)?;
    Ok(config)
}
