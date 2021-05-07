use core::f32;
use std::collections::HashMap;

use choices::{load_choices, Choice};
use config::config_struct::Config;
use draw_data::{DataType, DrawData};
use eyre::Result;
use ggez::event::{EventHandler, KeyCode, KeyMods};
use ggez::graphics::{self, Color, DrawMode, DrawParam, MeshBuilder, Rect, BLACK, WHITE};
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
        // self.create_nails();
        self.create_walls();
        self.create_left_collector();
    }

    fn create_choice_balls(&mut self) {
        for choice in &self.choices {
            let radius = self.config.choice_radius;
            let start_x = self.config.wall_width + self.config.choice_radius * 2.0;
            let end_x =
                self.config.width - self.config.wall_width - self.config.choice_radius * 2.0;
            let start_y = -(self.choices.len() as f32 * self.config.choice_radius);
            let end_y = 0.0;
            let position = Vector2::new_random(start_x..end_x, start_y..end_y);
            let id = self
                .physics
                .insert_ball(position, radius, self.config.bounciness);
            self.draw_data.insert_color(id, choice.color);
            self.draw_data.insert_type(id, DataType::Ball);
        }
    }

    fn insert_floor(&mut self) {
        let position = Vector2::new(self.config.width / 2.0, self.config.floor_position_y);
        let id = self
            .physics
            .insert_wall(position, self.config.width, self.config.floor_height);
        self.draw_data.insert_type(id, DataType::Wall);
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

    fn create_nails(&mut self) {
        let mut y = self.config.floor_position_y * 2.0;
        let space_between_x = self.config.width / self.config.nails_in_row as f32;
        let space_between_y = (self.config.height - y) / self.config.rows_of_nails as f32;
        dbg!(space_between_y);

        for y_count in 0..self.config.rows_of_nails {
            for x_count in 0..self.config.nails_in_row {
                let offset = if y_count % 2 == 0 {
                    space_between_x / 2.0
                } else {
                    0.0
                };
                let position = Vector2::new(space_between_x * x_count as f32 + offset, y);
                let id = self.physics.insert_nail(position, self.config.nail_radius);
                self.draw_data.insert_type(id, DataType::Nail);
            }
            y += space_between_y;
        }
    }

    fn create_walls(&mut self) {
        let left_position = Vector2::new(self.config.wall_width / 2.0, self.config.height / 2.0);
        let left_id = self.physics.insert_wall(
            left_position,
            self.config.wall_width,
            self.config.height * 2.0,
        );
        let right_position = Vector2::new(
            self.config.width - self.config.wall_width / 2.0,
            self.config.height / 2.0,
        );
        let right_id = self.physics.insert_wall(
            right_position,
            self.config.wall_width,
            self.config.height * 2.0,
        );
        self.draw_data.insert_type(left_id, DataType::Wall);
        self.draw_data.insert_type(right_id, DataType::Wall);
        self.draw_data.insert_rectangle(
            left_id,
            Rect::new(0.0, 0.0, self.config.wall_width, self.config.height),
        );
        self.draw_data.insert_rectangle(
            right_id,
            Rect::new(
                self.config.width - self.config.wall_width,
                0.0,
                self.config.wall_width,
                self.config.height,
            ),
        );
        self.draw_data.insert_color(left_id, self.config.wall_color);
        self.draw_data
            .insert_color(right_id, self.config.wall_color);
    }

    fn create_left_collector(&mut self) {
        let position = Vector2::new(
            self.config.width / 4.0,
            self.config.height - self.config.collector_offset_y,
        );
        let width = self.config.width / 2.0;
        let height = self.config.wall_width;
        let rotation = self.config.collector_rotation;
        let draw_type = DataType::Collector;
        let id = self.physics.insert_rotated_wall(
            position,
            width,
            height,
            rotation,
            -self.config.collector_rotation_offset,
        );
        let draw_rect = Rect::new(
            0.0,
            self.config.height - self.config.collector_offset_y,
            width + self.config.collector_rotation_offset,
            height,
        );

        self.draw_data.insert_rectangle(id, draw_rect);
        self.draw_data.insert_rotation(id, rotation);
        self.draw_data.insert_type(id, draw_type);
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
                DataType::Ball => {
                    let color = self.draw_data.get_color(id);
                    mesh_builder
                        .circle(
                            DrawMode::fill(),
                            [position.x, position.y],
                            self.config.choice_radius,
                            0.1,
                            color,
                        )
                        .circle(
                            DrawMode::stroke(2.0),
                            [position.x, position.y],
                            self.config.choice_radius,
                            0.1,
                            BLACK,
                        );
                }
                DataType::Wall => {
                    let color = self.draw_data.get_color(id);
                    let rect = self.draw_data.get_rectangle(id);
                    mesh_builder.rectangle(DrawMode::fill(), rect, color);
                }
                DataType::Nail => {
                    mesh_builder.circle(
                        DrawMode::fill(),
                        [position.x, position.y],
                        self.config.nail_radius,
                        1.0,
                        self.config.nail_color,
                    );
                }
                DataType::Collector => {
                    let rect = self.draw_data.get_rectangle(id);
                    let rotation = self.draw_data.get_rotation(id);
                    let mesh = MeshBuilder::new()
                        .rectangle(DrawMode::fill(), rect, self.config.wall_color)
                        .build(context)?;
                    graphics::draw(context, &mesh, DrawParam::new().rotation(rotation))?;
                }
                DataType::Unknown => {}
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
