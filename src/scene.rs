pub mod title_scene;
mod title_sound;
pub mod stage_scene;
mod stage_sound;
pub mod stage_select_scene;
mod stage_select_sound;

use ggez::{
    Context,
    GameResult,
};
use crate::input::Input;

pub trait Scene {

    fn update(&mut self, ctx: &mut Context, input: &Input) -> GameResult;
    fn draw(&self, ctx: &mut Context) -> GameResult;
    fn next_scene(&mut self, ctx: &mut Context) -> Option<Box<Scene>>;

}
