pub struct Velocity{
    pub x: f32,
    pub y: f32,
    epsilon: f32,
}

impl Velocity {

    pub fn new(x: f32, y: f32) -> Velocity {
        Velocity{
            x,
            y,
            epsilon: 0.05,
        }
    }

    /// - 現在の x に force を加える
    /// - 最高速度 max を超える場合、x に max を設定する
    pub fn add_force_x_with_max(&mut self, force: f32, max: f32) {
        self.x = add_force_with_max(self.x, force, max);
    }

    /// - 現在の y に force を加える
    /// - 最高速度 max を超える場合、x に max を設定する
    pub fn add_force_y_with_max(&mut self, force: f32, max: f32) {
        self.y = add_force_with_max(self.y, force, max);
    }

    /// - x 軸方向の摩擦を反映する
    /// - 摩擦の影響で速度が epsilon 以下になる場合、停止する
    pub fn reflect_friction_x(&mut self, friction: f32) {
        let next_x = self.x * friction;
        if (next_x * next_x) < (self.epsilon * self.epsilon) {
            self.x = 0.0;
        } else {
            self.x = next_x;
        }
    }

    pub fn add_force_y(&mut self, force: f32) {
        self.y += force;
    }

    pub fn stop_x(&mut self) {
        self.x = 0.0;
    }

    pub fn stop_y(&mut self) {
        self.y = 0.0;
    }

    pub fn goes_up(&self) -> bool {
        self.y < 0.0
    }

    pub fn goes_right(&self) -> bool {
        self.x > 0.0
    }

    pub fn goes_down(&self) -> bool {
        self.y > 0.0
    }

    pub fn goes_left(&self) -> bool {
        self.x < 0.0
    }

}

fn add_force_with_max(val: f32, force: f32, max: f32) -> f32 {
    let next_val = val + force;
    if (next_val * next_val) > (max * max) {
        if next_val > 0.0 {
            // 正方向
            max
        } else {
            // 負方向
            -max
        }
    } else {
        next_val
    }
}
