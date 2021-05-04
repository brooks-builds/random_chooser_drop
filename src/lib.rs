use std::collections::HashMap;

use choices::{load_choices, Choice};
use config::config_struct::Config;
use eyre::Result;
use ggez::event::EventHandler;
use ggez::graphics::{self, Color, DrawMode, DrawParam, MeshBuilder};
use helpers::vector2::Vector2;
use physics::Physics;

mod choices;
pub mod config;
mod helpers;
mod physics;

pub struct MainState {
    config: Config,
    choices: Vec<Choice>,
    physics: Physics,
    colors: HashMap<u128, Color>,
}

impl MainState {
    pub fn new(config: Config) -> Result<Self> {
        let choices = load_choices(&config)?;
        let physics = Physics::new(&config);

        Ok(Self {
            config,
            choices,
            physics,
            colors: HashMap::new(),
        })
    }

    pub fn setup(&mut self) {
        self.create_choice_balls();
        self.insert_floor();
    }

    fn create_choice_balls(&mut self) {
        for choice in &self.choices {
            let radius = self.config.choice_radius;
            let position = Vector2::new_random(
                self.config.choice_start_x_min..self.config.choice_start_x_max,
                self.config.choice_start_y_min..self.config.choice_start_y_max,
            );
            let id = self
                .physics
                .insert_ball(position, radius, self.config.bounciness);
            self.colors.insert(id, choice.color);
        }
    }

    fn insert_floor(&mut self) {
        // inserting the floor of the collector
        // This will be removed when the simulations starts
        // self.physics.insert_wall(position, width, height)
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _context: &mut ggez::Context) -> ggez::GameResult {
        self.physics.update();
        Ok(())
    }

    fn draw(&mut self, context: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(context, self.config.background_color);
        let mut mesh_builder = MeshBuilder::new();
        for (_handle, body) in self.physics.bodies.iter_active_dynamic() {
            let position = body.world_com;
            let id = body.user_data;
            let color = self.colors.get(&id).unwrap();
            mesh_builder.circle(
                DrawMode::fill(),
                [position.x, position.y],
                self.config.choice_radius,
                0.1,
                *color,
            );
        }
        let mesh = mesh_builder.build(context)?;
        graphics::draw(context, &mesh, DrawParam::new())?;
        graphics::present(context)
    }
}
