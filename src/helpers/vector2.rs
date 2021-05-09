use std::ops::Range;

use rand::{thread_rng, Rng};

pub struct Vector2 {
    data: nalgebra::Vector2<f32>,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        let mut vector2 = nalgebra::Vector2::default();
        vector2.x = x;
        vector2.y = y;

        Self { data: vector2 }
    }

    pub fn new_random(x_range: Range<f32>, y_range: Range<f32>) -> Self {
        let mut rng = thread_rng();
        Self::new(rng.gen_range(x_range), rng.gen_range(y_range))
    }

    pub fn get_nalgebra(&self) -> &nalgebra::Vector2<f32> {
        &self.data
    }

    pub fn get_x_mut(&mut self) -> &mut f32 {
        &mut self.data.x
    }

    pub fn to_nalgebra(&self) -> nalgebra::Vector2<f32> {
        self.data
    }
}
