use std::fs::File;
use std::io::Read;

use eyre::Result;
use ggez::graphics::Color;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Choice {
    pub name: String,
    #[serde(
        with = "crate::helpers::serde_color",
        default = "crate::helpers::serde_default_color::serde_default_color"
    )]
    pub color: Color,
}

pub fn load_choices_from_json(path: String) -> Result<Vec<Choice>> {
    let mut choices_file = File::open(&path)?;
    let mut choices_json = String::new();
    choices_file.read_to_string(&mut choices_json)?;
    let choices: Vec<Choice> = serde_json::from_str(&choices_json)?;

    Ok(choices)
}

pub fn load_choices_from_csv(_path: String) -> Result<Vec<Choice>> {
    Ok(vec![])
}
