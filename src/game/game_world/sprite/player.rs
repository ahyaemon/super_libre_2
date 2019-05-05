use ggez::Context;
use ggez::GameResult;
use ggez::graphics;
use ggez::graphics::Image;

use crate::game::input::Input;
use crate::game::input::InputType;
use crate::game::game_world::map::Map;
use crate::game::game_world::sprite::Sprite;
use crate::game::game_world::sprite::physics::{Velocity, LinearCollider};
use crate::game::game_world::sprite::physics::Position;
use crate::game::game_world::sprite::physics::Figure;


#[derive(PartialEq)]
enum PlayerStateType {
    Stand,
    _Walk,
    _Dash,
    _Jump,
    _Air,
    Dead,
}

pub struct Player {
    image: Image,
    figure: Figure,
    position: Position,
    state: PlayerStateType,

    // 移動系パラメータ
    velocity: Velocity,
    walk_power: f32,
    max_walk_speed: f32,
    _dash_power: f32,
    _max_dash_speed: f32,
    _jump_power: f32,
    _max_fall_speed: f32,
    friction: f32,
    max_fall_speed: f32,

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
                figure: Figure::new(96.0, 64.0),
                position: Position::new(100.0, 100.0),
                state: PlayerStateType::Stand,

                velocity: Velocity::new(0.0, 0.0),
                walk_power: 2.0,
                max_walk_speed: 10.0,
                _dash_power: 4.0,
                _max_dash_speed: 20.0,
                _jump_power: 10.0,
                _max_fall_speed: 10.0,
                friction: 0.9,
                max_fall_speed: 20.0,

                gravity: 0.5,
                is_ground: false,

                is_dead: false,
            }
        )
    }

    pub fn input(&mut self, _ctx: &mut Context, input: &Input) {
        // 歩き（右）
        if input.pressing.contains(&InputType::Right) {
            self.velocity.add_force_x_with_max(self.walk_power, self.max_walk_speed);
        }

        // 歩き（左）
        if input.pressing.contains(&InputType::Left) {
            self.velocity.add_force_x_with_max(self.walk_power, self.max_walk_speed);
        }
    }

    pub fn update(&mut self, _ctx: &mut Context) {
        // 横移動の減速
        self.velocity.reflect_friction_x(self.friction);

        // 重力
        if !self.is_ground {
            self.velocity.add_force_y_with_max(self.gravity, self.max_fall_speed);
        }

        // 最終的な位置決定
        self.position.add_velocity(&self.velocity);

        // 死亡判定
        // TODO: 全てのスプライトの update が終了してから判定したほうがいい？
        if self.position.y > 664.0 { // TODO: 固定値 (画面下の死亡判定領域)
            self.state = PlayerStateType::Dead;
        }

    }

    pub fn map_collision(&mut self, map: &Map) {
        let mut sprite_top = None;
        let mut sprite_right = None;
        let mut sprite_bottom = None;
        let mut sprite_left = None;
        {
            let collider = LinearCollider::new(&self.figure, &self.position, &self.velocity);
            if self.velocity.goes_up() {
                sprite_top = self.collide_top(map, &collider);
            }
            if self.velocity.goes_down() {
                sprite_bottom = self.collide_bottom(map, &collider);
            }
            if self.velocity.goes_right() {
                sprite_right = self.collide_right(map);
            }
            if self.velocity.goes_left() {
                sprite_left = self.collide_left(map);
            }
        }

        // 上に障害物がある場合
        if let Some(sprite) = sprite_top {
            self.velocity.stop_y();
            let new_y = sprite.get_bottom();
            self.position.set_y(new_y);
        }

        // 右に障害物がある場合
        if let Some(sprite) = sprite_right {
            self.velocity.stop_x();
            let new_x = sprite.get_left() - 48.0;
            self.position.set_x(new_x);
        }

        // 下に障害物がある場合
        // 地上に降り立ったこととする。
        if let Some(sprite) = sprite_bottom {
            // 下と衝突
            self.is_ground = true;
            self.velocity.stop_y();

            // player 足元の y 座標を sprite の頂点座標に合わせる
            let new_y = sprite.get_top() - self.figure.width;
            self.position.set_y(new_y);
        }

        // 左に障害物がある場合
        if let Some(sprite) = sprite_left {
            self.velocity.stop_x();
            let new_x = sprite.get_right();
            self.position.set_x(new_x);
        }
    }

    /// 上にぶつかったスプライトがある場合はその参照を返す
    fn collide_top<'a, 'b>(&self, map: &'a Map, collider: &'b LinearCollider) -> Option<&'a Sprite> {
        let cell_coordinates = collider.create_coordinates_up(10);
        for (irow, icol) in cell_coordinates {
            if let Some(sprite_ref) = map.get_sprite(irow, icol) {
                return Some(sprite_ref);
            }
        }
        None
    }

    /// 右にぶつかったスプライトがある場合はその参照を返す
    fn collide_right<'a>(&self, _map: &'a Map) -> Option<&'a Sprite> {
        None
    }

    /// 下にぶつかったスプライトがある場合はその参照を返す
    fn collide_bottom<'a, 'b>(&self, map: &'a Map, collider: &'b LinearCollider) -> Option<&'a Sprite> {
        None
    }

    /// 左にぶつかったスプライトがある場合はその参照を返す
    fn collide_left<'a>(&self, _map: &'a Map) -> Option<&'a Sprite> {
        None
    }

    pub fn sprite_collision(&self) {}

    pub fn is_dead(&self) -> bool {
        self.state == PlayerStateType::Dead
    }

    pub fn draw(&self, ctx: &mut Context) {
        let _ = graphics::draw(ctx, &self.image, (self.position.to_point(),));
    }

}
