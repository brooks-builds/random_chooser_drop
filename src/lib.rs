use std::collections::HashMap;

use choices::{load_choices, Choice};
use config::config_struct::Config;
use draw_data::DrawData;
use eyre::Result;
use ggez::event::{EventHandler, KeyCode, KeyMods};
use ggez::graphics::{self, Color, DrawMode, DrawParam, MeshBuilder, Rect, WHITE};
use ggez::Context;
use helpers::vector2::Vector2;
use physics::Physics;

mod choices;
pub mod config;
mod draw_data;
mod helpers;
mod physics;

pub struct MainState {
    config: Config,
    choices: Vec<Choice>,
    physics: Physics,
    draw_data: DrawData,
    floor_id: Option<u128>,
}

impl MainState {
    pub fn new(config: Config) -> Result<Self> {
        let choices = load_choices(&config)?;
        let physics = Physics::new(&config);

        Ok(Self {
            config,
            choices,
            physics,
            draw_data: DrawData::new(),
            floor_id: None,
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
            self.draw_data.insert_color(id, choice.color);
            self.draw_data.insert_type(id, draw_data::DataType::Ball);
        }
    }

    fn insert_floor(&mut self) {
        let position = Vector2::new(self.config.width / 2.0, self.config.floor_position_y);
        let id = self
            .physics
            .insert_wall(position, self.config.width, self.config.floor_height);
        self.draw_data.insert_type(id, draw_data::DataType::Floor);
        self.draw_data.insert_color(id, self.config.floor_color);
        let rect = Rect::new(
            0.0,
            self.config.floor_position_y - self.config.floor_height / 2.0,
            self.config.width,
            self.config.floor_height,
        );
        self.draw_data.insert_rectangle(id, rect);
        self.floor_id = Some(id);
    }

    fn remove_floor(&mut self) {
        let floor_id = if let Some(id) = self.floor_id {
            id
        } else {
            return;
        };

        let handle = self.physics.get_rigid_body_handle(floor_id).unwrap();
        self.physics.remove(handle);
        self.floor_id = None;
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
        for (_handle, body) in self.physics.bodies.iter() {
            let position = body.world_com;
            let id = body.user_data;
            match self.draw_data.get_type(id) {
                draw_data::DataType::Ball => {
                    let color = self.draw_data.get_color(id);
                    mesh_builder.circle(
                        DrawMode::fill(),
                        [position.x, position.y],
                        self.config.choice_radius,
                        0.1,
                        color,
                    );
                }
                draw_data::DataType::Floor => {
                    let color = self.draw_data.get_color(id);
                    let rect = self.draw_data.get_rectangle(id);
                    mesh_builder.rectangle(DrawMode::fill(), rect, color);
                }
                draw_data::DataType::Unknown => {}
            }
        }

        let mesh = mesh_builder.build(context)?;
        graphics::draw(context, &mesh, DrawParam::new())?;
        graphics::present(context)
    }

    fn key_down_event(
        &mut self,
        context: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        if let KeyCode::Space = keycode {
            self.remove_floor();
        }
    }
}
