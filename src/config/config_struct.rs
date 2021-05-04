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
    pub gravity: f32,
    pub choice_radius: f32,
    pub bounciness: f32,
    pub choice_start_x_min: f32,
    pub choice_start_x_max: f32,
    pub choice_start_y_min: f32,
    pub choice_start_y_max: f32,
}
