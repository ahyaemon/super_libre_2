use ggez::Context;
use ggez::GameResult;
use ggez::event::EventHandler;
use ggez::event::KeyCode;
use ggez::event::KeyMods;
use ggez::graphics;

use super::scene::Scene;
use super::scene::TitleScene;
use super::input::Input;


pub struct GameState {
    scene: Box<Scene>,
    input: Input,
}

impl GameState {

    pub fn new(ctx: &mut Context) -> GameResult<GameState> {
        let title_scene = Box::new(TitleScene::new(ctx)?);
        let input = Input::new();

        let game_state = GameState {
            scene: title_scene,
            input,
        };
        Ok(game_state)
    }

}

impl EventHandler for GameState {

    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // シーンの遷移
        if let Some(scene) = self.scene.next_scene(ctx) {
            self.scene = scene;
        }

        // シーンのアップデート
        let _scene_update_result = self.scene.update(ctx, &self.input)?;
//        self.input.show();

        self.input.clear();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);
        let _scene_draw_result = self.scene.draw(ctx)?;
        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        self.input.key_down(keycode);
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods) {
        self.input.key_up(keycode);
    }

}

