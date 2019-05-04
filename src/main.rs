use ggez::GameResult;
use ggez::event;

use std::env;
use std::path;

mod game;
use self::game::GameState;

mod game_data;
use self::game_data::GameData;

#[macro_use]
extern crate serde_derive;


pub fn main() -> GameResult {
    // game_data を設定ファイルから取得
    // リリースビルド時は設定ファイルからではなく、直接 rust ファイルに展開したい
    let game_data = GameData::new()?;

    // リソースの読み込み設定
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let window_setup = ggez::conf::WindowSetup{
        title: game_data.title,
        .. Default::default()
    };

    let window_mode = ggez::conf::WindowMode {
        width: game_data.window_width,
        height: game_data.window_height,
        .. Default::default()
    };

    let cb = ggez::ContextBuilder::new("SUPER LIBRE 2", "ahyaemon")
        .add_resource_path(resource_dir)
        .window_mode(window_mode)
        .window_setup(window_setup);
    let (ctx, events_loop) = &mut cb.build()?;
    let state = &mut GameState::new(ctx).unwrap();

    // ゲーム開始
    event::run(ctx, events_loop, state)
}
