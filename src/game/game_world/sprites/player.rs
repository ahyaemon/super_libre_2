use ggez::Context;
use ggez::GameResult;
use ggez::graphics;
use ggez::graphics::Image;

use cgmath::Point2;

use crate::game::input::Input;
use crate::game::input::InputType;


enum _PlayerStateType {
    Stand,
    Walk,
    Dash,
    Jump,
    Air,
}

pub struct Player {
    image: Image,
    position: Point2<f32>,

    // 移動系パラメータ
    velocity: Point2<f32>,
    walk_power: f32,
    max_walk_speed: f32,
    _dash_power: f32,
    _max_dash_speed: f32,
    _jump_power: f32,
    _max_fall_speed: f32,
    friction: f32,
}

impl Player {

    pub fn new(ctx: &mut Context) -> GameResult<Player> {
        Ok(
            Player {
                image: Image::new(ctx, "/img/player/stand01.png")?,
                position: Point2::new(100.0, 100.0),

                velocity: Point2::new(0.0, 0.0),
                walk_power: 2.0,
                max_walk_speed: 10.0,
                _dash_power: 4.0,
                _max_dash_speed: 20.0,
                _jump_power: 10.0,
                _max_fall_speed: 10.0,
                friction: 0.9,
            }
        )
    }

    pub fn input(&mut self, _ctx: &mut Context, input: &Input) {
        // 歩き（右）
        if input.pressing.contains(&InputType::Right) {
            self.velocity.x += self.walk_power;
            // 最高速度を超えないように調整
            if self.velocity.x > self.max_walk_speed {
                self.velocity.x = self.max_walk_speed;
            }
        }

        // 歩き（左）
        if input.pressing.contains(&InputType::Left) {
            self.velocity.x -= self.walk_power;
            // 最高速度を超えないように調整
            if self. velocity.x < -self.max_walk_speed {
                self.velocity.x = -self.max_walk_speed;
            }
        }
    }

    pub fn update(&mut self, _ctx: &mut Context) {
        // 横移動の減速
        match self.velocity.x {
            velx if 0.5 <= velx => {
                self.velocity.x = velx * self.friction;
            }
            velx if 0.0 < velx  => {
                self.velocity.x = 0.0;
            }
            velx if velx == 0.0 => {

            }
            velx if -0.5 < velx => {
                self.velocity.x = 0.0;
            }
            velx if velx <= -0.5 => {
                self.velocity.x = velx * self.friction;
            }
            _ => {}
        }


        // 最終的な位置決定
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
    }

    pub fn draw(&self, ctx: &mut Context) {
        let _ = graphics::draw(ctx, &self.image, (self.position,));
    }

}
