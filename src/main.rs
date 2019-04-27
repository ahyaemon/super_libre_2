use ggez::GameResult;
use ggez::event;

use std::env;
use std::path;

mod game;
use self::game::GameState;


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
