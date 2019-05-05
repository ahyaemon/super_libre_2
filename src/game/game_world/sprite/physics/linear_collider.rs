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

    /// - y 軸方向のイテレーション用ベクトルを生成する
    /// - step の間隔でイテレーションする
    /// - y の最後を含む
    pub fn create_vec_y(&self, step:i32) -> Vec<i32> {

    }

}
