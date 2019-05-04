pub mod background;

mod player;
pub use self::player::Player;

pub mod physics;

use ggez::Context;

pub trait Sprite {

    fn update(&mut self, ctx: &mut Context);
    fn draw(&self, ctx: &mut Context);

    fn get_top(&self) -> f32;
    fn get_right(&self) -> f32;
    fn get_bottom(&self) -> f32;
    fn get_left(&self) -> f32;

}
