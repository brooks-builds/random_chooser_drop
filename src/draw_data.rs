use std::collections::HashMap;

use ggez::graphics::{Color, Rect};

pub struct DrawData {
    colors: HashMap<u128, Color>,
    types: HashMap<u128, DataType>,
    rectangles: HashMap<u128, Rect>,
    rotations: HashMap<u128, f32>,
}

impl DrawData {
    pub fn new() -> Self {
        Self {
            colors: HashMap::new(),
            types: HashMap::new(),
            rectangles: HashMap::new(),
            rotations: HashMap::new(),
        }
    }

    pub fn insert_color(&mut self, id: u128, color: Color) {
        self.colors.insert(id, color);
    }

    pub fn insert_type(&mut self, id: u128, data_type: DataType) {
        self.types.insert(id, data_type);
    }

    pub fn insert_rectangle(&mut self, id: u128, rect: Rect) {
        self.rectangles.insert(id, rect);
    }

    pub fn insert_rotation(&mut self, id: u128, rotation: f32) {
        self.rotations.insert(id, rotation);
    }

    pub fn get_color(&mut self, id: u128) -> Color {
        *self.colors.get(&id).unwrap()
    }

    pub fn get_type(&self, id: u128) -> DataType {
        if let Some(data_type) = self.types.get(&id) {
            *data_type
        } else {
            DataType::Unknown
        }
    }

    pub fn get_rectangle(&self, id: u128) -> Rect {
        *self.rectangles.get(&id).unwrap()
    }

    pub fn get_rotation(&self, id: u128) -> f32 {
        *self.rotations.get(&id).unwrap()
    }
}

#[derive(Clone, Copy)]
pub enum DataType {
    Ball,
    Collector,
    Nail,
    Wall,
    Unknown,
}
