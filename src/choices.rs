use std::fs::File;
use std::io::Read;

use eyre::Result;
use ggez::graphics::Color;
use serde::{Deserialize, Serialize};

use crate::helpers::serde_color;

#[derive(Serialize, Deserialize, Debug)]
pub struct Choice {
    pub name: String,
    #[serde(default = "default_color")]
    pub red: u8,
    #[serde(default = "default_color")]
    pub green: u8,
    #[serde(default = "default_color")]
    pub blue: u8,
    #[serde(default = "default_alpha")]
    pub alpha: u8,
}

impl Choice {
    pub fn color(&self) -> Color {
        Color::from_rgba(self.red, self.green, self.blue, self.alpha)
    }
}

pub fn load_choices_from_json(path: String) -> Result<Vec<Choice>> {
    let mut choices_file = File::open(&path)?;
    let mut choices_json = String::new();
    choices_file.read_to_string(&mut choices_json)?;
    let choices: Vec<Choice> = serde_json::from_str(&choices_json)?;

    Ok(choices)
}

pub fn load_choices_from_csv(path: String) -> Result<Vec<Choice>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut choices = vec![];
    for result in reader.deserialize() {
        let choice: Choice = result?;
        choices.push(choice);
    }
    Ok(choices)
}

pub fn load_choices_from_stdin() -> Result<Vec<Choice>> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    Ok(serde_json::from_str(&input)?)
}

fn default_color() -> u8 {
    0
}

fn default_alpha() -> u8 {
    255
}
