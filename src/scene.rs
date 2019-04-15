pub mod title_scene;
mod title_sound;

use ggez::{
    Context,
    GameResult,
};
use crate::input::Input;

pub trait Scene {
    fn update(&mut self, input: &Input) -> GameResult;
    fn draw(&self, ctx: &mut Context) -> GameResult;
}
