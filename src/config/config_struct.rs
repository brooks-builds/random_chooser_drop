use ggez::graphics::Color;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub title: String,
    pub width: f32,
    pub height: f32,
    pub vsync: bool,
    #[serde(with = "crate::helpers::serde_color")]
    pub background_color: Color,
    pub choices_path: String,
}
