use std::fs::File;
use std::io::Read;

use eyre::Result;
use ggez::graphics::Color;
use serde::{Deserialize, Serialize};

use crate::config::config_struct::Config;

#[derive(Serialize, Deserialize, Debug)]
pub struct Choice {
    name: String,
    #[serde(
        with = "crate::helpers::serde_color",
        default = "crate::helpers::serde_default_color::serde_default_color"
    )]
    color: Color,
}

pub fn load_choices(config: &Config) -> Result<Vec<Choice>> {
    let mut choices_file = File::open(&config.choices_path)?;
    let mut choices_json = String::new();
    choices_file.read_to_string(&mut choices_json)?;
    let choices: Vec<Choice> = serde_json::from_str(&choices_json)?;

    Ok(choices)
}
