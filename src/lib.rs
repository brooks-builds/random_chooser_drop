use choices::{load_choices, Choice};
use config::config_struct::Config;
use eyre::Result;
use ggez::event::EventHandler;
use ggez::graphics;

mod choices;
pub mod config;
mod helpers;

#[derive(Debug)]
pub struct MainState {
    config: Config,
    choices: Vec<Choice>,
}

impl MainState {
    pub fn new(config: Config) -> Result<Self> {
        let choices = load_choices(&config)?;
        Ok(Self { config, choices })
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _context: &mut ggez::Context) -> ggez::GameResult {
        Ok(())
    }

    fn draw(&mut self, context: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(context, self.config.background_color);
        graphics::present(context)
    }
}
