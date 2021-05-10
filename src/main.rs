use eyre::Result;
use ggez::conf::{WindowMode, WindowSetup};
use ggez::{event, ContextBuilder};
use random_chooser_drop::config::load_config;
use random_chooser_drop::MainState;

const DEFAULT_CONFIG_FILE_PATH: &str = "config.json";
const DEFAULT_CHOICES_PATH: &str = "choices.json";
const DEFAULT_CHOICE_FILE_TYPE: &str = "json";

fn main() -> Result<()> {
    let mut arguments = pico_args::Arguments::from_env();
    let config_path = arguments
        .opt_value_from_str("--config")?
        .unwrap_or_else(|| DEFAULT_CONFIG_FILE_PATH.to_owned());
    let choices_path = arguments
        .opt_value_from_str("--choices")?
        .unwrap_or_else(|| DEFAULT_CHOICES_PATH.to_owned());

    let choice_file_type = arguments
        .opt_value_from_str("--file-type")?
        .unwrap_or_else(|| DEFAULT_CHOICE_FILE_TYPE.to_owned());

    let config = load_config(config_path)?;
    let window_mode = WindowMode::default().dimensions(config.width, config.height);
    let window_setup = WindowSetup::default()
        .title(&config.title)
        .vsync(config.vsync);
    let (mut context, mut event_loop) = ContextBuilder::new("random_chooser_drop", "Brookzerker")
        .window_mode(window_mode)
        .window_setup(window_setup)
        .build()?;
    let mut main_state = MainState::new(config, choices_path, choice_file_type)?;
    main_state.setup();
    event::run(&mut context, &mut event_loop, &mut main_state)?;
    Ok(())
}
