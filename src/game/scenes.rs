mod title;
pub use self::title::title_scene::TitleScene;

mod stage_select;
pub use self::stage_select::stage_select_scene::StageSelectScene;

mod stage;
pub use self::stage::stage_scene::StageScene;

use ggez::Context;
use ggez::GameResult;

use crate::game::input::Input;


pub trait Scene {

    fn update(&mut self, ctx: &mut Context, input: &Input) -> GameResult;
    fn draw(&self, ctx: &mut Context) -> GameResult;
    fn next_scene(&mut self, ctx: &mut Context) -> Option<Box<Scene>>;

}
