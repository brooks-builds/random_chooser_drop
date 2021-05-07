pub struct Point {
    x: f32,
    y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl Into<rapier2d::math::Point<f32>> for Point {
    fn into(self) -> rapier2d::math::Point<f32> {
        nalgebra::Point2::new(self.x, self.y)
    }
}
