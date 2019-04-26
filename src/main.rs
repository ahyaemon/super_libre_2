use ggez::{
    Context,
    GameResult,
    event::{
        self,
        EventHandler,
        KeyCode,
        KeyMods,
    },
    graphics,
};
use std::{
    env,
    path,
};
mod scene;
use scene::{
    Scene,
    title_scene::TitleScene,
};
mod stext;
mod cursor;
mod sprite;
mod input;
use input::Input;

struct GameState {
    scene: Box<Scene>,
    input: Input,
}

impl GameState {

    fn new(ctx: &mut Context) -> GameResult<GameState> {
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

pub fn main() -> GameResult {
    // リソースの読み込み設定
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let cb = ggez::ContextBuilder::new("SUPER LIBRE 2", "ahyaemon").add_resource_path(resource_dir);
    let (ctx, events_loop) = &mut cb.build()?;
    let state = &mut GameState::new(ctx).unwrap();

    // ゲーム開始
    event::run(ctx, events_loop, state)
}
