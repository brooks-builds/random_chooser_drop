use ggez::graphics::{Font, Scale, Text};
use ggez::Context;

pub fn create_winner_text(context: &mut Context, name: &str) -> Text {
    let mut winner = Text::new(format!("{} Won!!!", name));
    let font = Font::default();
    let scale = Scale::uniform(72.0);
    winner.set_font(font, scale);
    winner
}
