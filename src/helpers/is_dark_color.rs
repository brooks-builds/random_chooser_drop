use ggez::graphics::Color;

pub fn is_dark_color(color: &Color) -> bool {
    let average = (color.r + color.g + color.b) / 3.0;
    average < 0.5
}
