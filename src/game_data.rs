use std::fs;

use ggez::GameResult;


#[derive(Deserialize)]
pub struct GameData {
    pub title: String,
    pub window_width: f32,
    pub window_height: f32,
}

impl GameData {

    pub fn new() -> GameResult<GameData> {
        let toml = fs::read_to_string("resources/game_data.toml")?;
        Ok(toml::from_str(&toml)?)
    }

}
