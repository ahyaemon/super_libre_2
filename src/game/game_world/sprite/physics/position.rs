use cgmath::Point2;
use super::Velocity;

pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {

    pub fn new(x: f32, y: f32) -> Position {
        Position{ x, y }
    }

    pub fn to_point(&self) -> Point2<f32> {
        Point2::new(self.x, self.y)
    }

    pub fn add_velocity(&mut self, velocity: &Velocity) {
        self.x += velocity.x;
        self.y += velocity.y;
    }

    pub fn set_x(&mut self, x: f32) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: f32) {
        self.y = y;
    }

}
