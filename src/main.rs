use eyre::Result;
use ggez::conf::{WindowMode, WindowSetup};
use ggez::{event, ContextBuilder};
use random_chooser_drop::config::load_config;
use random_chooser_drop::MainState;

const CONFIG_FILE_PATH: &str = "config.json";

fn main() -> Result<()> {
    let config = load_config(CONFIG_FILE_PATH)?;
    let window_mode = WindowMode::default().dimensions(config.width, config.height);
    let window_setup = WindowSetup::default()
        .title(&config.title)
        .vsync(config.vsync);
    let (mut context, mut event_loop) = ContextBuilder::new("random_chooser_drop", "Brookzerker")
        .window_mode(window_mode)
        .window_setup(window_setup)
        .build()?;
    let mut main_state = MainState::new(config)?;
    dbg!(&main_state);
    event::run(&mut context, &mut event_loop, &mut main_state)?;
    Ok(())
}
