use ggez::Context;
use ggez::GameResult;
use ggez::graphics;
use ggez::graphics::Image;

use cgmath::Point2;

use crate::game::input::Input;
use crate::game::input::InputType;
use crate::game::game_world::map::Map;
use crate::game::game_world::sprite::Sprite;
use std::slice::Iter;


#[derive(PartialEq)]
enum PlayerStateType {
    Stand,
    Walk,
    Dash,
    Jump,
    Air,
    Dead,
}

pub struct Player {
    image: Image,
    position: Point2<f32>,
    state: PlayerStateType,

    // 移動系パラメータ
    velocity: Point2<f32>,
    walk_power: f32,
    max_walk_speed: f32,
    _dash_power: f32,
    _max_dash_speed: f32,
    _jump_power: f32,
    _max_fall_speed: f32,
    friction: f32,

    // 重力
    gravity: f32,
    is_ground: bool,

    is_dead: bool,
}

impl Player {

    pub fn new(ctx: &mut Context) -> GameResult<Player> {
        Ok(
            Player {
                image: Image::new(ctx, "/img/player/stand01.png")?,
                position: Point2::new(100.0, 100.0),
                state: PlayerStateType::Stand,

                velocity: Point2::new(0.0, 0.0),
                walk_power: 2.0,
                max_walk_speed: 10.0,
                _dash_power: 4.0,
                _max_dash_speed: 20.0,
                _jump_power: 10.0,
                _max_fall_speed: 10.0,
                friction: 0.9,

                gravity: 0.5,
                is_ground: false,

                is_dead: false,
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

        // 重力
        if !self.is_ground {
            self.velocity.y += self.gravity;
        }

        // 最終的な位置決定
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;

        // 死亡判定
        // TODO: 全てのスプライトの update が終了してから判定したほうがいい？
        if self.position.y > 664.0 {
            self.state = PlayerStateType::Dead;
        }

    }

    pub fn map_collision(&mut self, map: &Map) {
        // 横方向あたり判定
        if self.velocity.x >= 0.0 {
            if let Some(sprite) = self.collide_right(map) {
                self.velocity.x = 0.0;
                let new_x = sprite.get_left() - 48.0;
                self.position.x = new_x;
            }
        } else {
            if let Some(sprite) = self.collide_left(map) {
                self.velocity.x = 0.0;
                let new_x = sprite.get_right();
                self.position.x = new_x;
            }
        }

        // 縦方向あたり判定
        // 下にあたり判定があった場合のみ、地上に降り立ったこととする。
        if self.velocity.y >= 0.0 {
            if let Some(sprite) = self.collide_bottom(map) {
                // 下と衝突
                self.is_ground = true;
                self.velocity.y = 0.0;

                // player 足元の y 座標を sprite の頂点座標に合わせる
                let new_y = sprite.get_top() - 64.0;
                self.position.y = new_y;
            }
        } else {
            if let Some(sprite) = self.collide_top(map) {
                self.velocity.x = 0.0;
                let new_y = sprite.get_bottom();
                self.position.y = new_y;
            }
        }

    }

    /// 上にぶつかったスプライトがある場合はその参照を返す
    fn collide_top<'a>(&self, map: &'a Map) -> Option<&'a Sprite> {
        None
    }

    /// 右にぶつかったスプライトがある場合はその参照を返す
    fn collide_right<'a>(&self, map: &'a Map) -> Option<&'a Sprite> {
        None
    }

    /// 下にぶつかったスプライトがある場合はその参照を返す
    fn collide_bottom<'a>(&self, map: &'a Map) -> Option<&'a Sprite> {
        // 判定する数だけアローを用意する
        let beg = self.position.y + 96.0;
        let end = self.position.y + 96.0 + self.velocity.y;
        let vec = f32_to_i32vec_contains_end(beg, end, 10);
        let itr = vec.iter();

        match self.velocity.x {
            velx if 0.05 >= velx * velx => {
                // 真下に移動している場合は、x 固定
                for y in itr {
                    let x = self.position.x;
                    let irow = ((*y as f32) / 64.0).round() as usize;
                    let icol = (x / 64.0).round() as usize;
                    if let Some(sprite_ref) = map.get_sprite(irow, icol) {
                        return Some(sprite_ref)
                    }
                }
            }
            _ => {
                // 係数の用意
                let (a, b) = calc_coef(&self.position, &self.velocity);
                for y in itr {
                    let x = (*y as f32) / a - b / a;
                    let irow = ((*y as f32) / 64.0).round() as usize;
                    let icol = (x / 64.0).round() as usize;
                    if let Some(sprite_ref) = map.get_sprite(irow, icol) {
                        return Some(sprite_ref)
                    }
                }
            }
        }


        None
    }

    /// 左にぶつかったスプライトがある場合はその参照を返す
    fn collide_left<'a>(&self, map: &'a Map) -> Option<&'a Sprite> {
        None
    }

    pub fn sprite_collision(&self) {}

    pub fn is_dead(&self) -> bool {
        self.state == PlayerStateType::Dead
    }

    pub fn draw(&self, ctx: &mut Context) {
        let _ = graphics::draw(ctx, &self.image, (self.position,));
    }

}

fn calc_coef(position: &Point2<f32>, velocity: &Point2<f32>) -> (f32, f32) {
    let x0 = position.x;
    let y0 = position.y;
    let x1 = position.x + velocity.x;
    let y1 = position.y + velocity.y;
    let a = (y1 - y0) / (x1 - x0);
    let b = (x1 * y0 - x0 * y1) / (x1 - x0);
    (a, b)
}

fn f32_to_i32vec_contains_end(beg: f32, end: f32, step: i32) -> Vec<i32> {
    let i_beg = beg.round() as i32;
    let i_end = end.round() as i32;
    let mut vec = (i_beg..i_end)
        .filter(|i| i % step == 0)
        .collect::<Vec<i32>>();
    vec.push(i_end);
    vec
}
