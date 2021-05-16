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
    pub gravity: f32,
    pub choice_radius: f32,
    pub bounciness: f32,
    pub floor_position_y: f32,
    pub floor_height: f32,
    #[serde(with = "crate::helpers::serde_color")]
    pub floor_color: Color,
    pub nails_in_row: u8,
    pub rows_of_nails: u8,
    pub nail_radius: f32,
    #[serde(with = "crate::helpers::serde_color")]
    pub nail_color: Color,
    pub wall_width: f32,
    #[serde(with = "crate::helpers::serde_color")]
    pub wall_color: Color,
    pub collector_offset_y: f32,
    pub collector_rotation: f32,
    pub collector_rotation_offset: f32,
    #[serde(with = "crate::helpers::serde_color")]
    pub winning_background_color_light: Color,
    #[serde(with = "crate::helpers::serde_color")]
    pub winning_background_color_dark: Color,
    pub winning_background_color_alpha: f32,
    #[serde(default = "default_use_stdin")]
    pub use_stdin: bool,
}

fn default_use_stdin() -> bool {
    false
}
