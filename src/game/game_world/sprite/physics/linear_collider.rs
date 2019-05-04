use super::Position;
use super::Velocity;


pub struct LinearCollider<'a> {
    position: &'a Position,
    velocity: &'a Velocity,
}

impl<'a> LinearCollider<'a> {

    pub fn new(position: &'a Position, velocity: &'a Velocity) -> LinearCollider<'a> {
        LinearCollider{ position, velocity }
    }

}
