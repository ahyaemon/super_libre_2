use ggez::Context;
use ggez::GameResult;
use ggez::audio::SoundSource;

use crate::game::scene::Scene;
use crate::game::scene::StageSelectScene;
use crate::game::input::Input;
use crate::game::game_world::sprite::Player;

use super::stage_sound::StageSound;
use crate::game::game_world::map::Map;


pub struct StageScene {
    player: Player,
    sound: StageSound,
    map: Map,
}

impl StageScene {

    pub fn new(ctx: &mut Context) -> GameResult<StageScene> {
        let mut scene = StageScene {
            player: Player::new(ctx)?,
            sound: StageSound::new(ctx)?,
            map: Map::from_filename(ctx, "resources/map/1-1.tsv")?,
        };
        let _ = scene.sound.bgm.play();
        Ok(scene)
    }

}

impl Scene for StageScene {

    fn update(&mut self, ctx: &mut Context, input: &Input) -> GameResult {
        self.player.input(ctx, input);
        self.player.update(ctx);
        self.map.update(ctx);
        self.player.map_collision(&self.map);
        Ok(())
    }

    fn draw(&self, ctx: &mut Context) -> GameResult {
        self.player.draw(ctx);
        self.map.draw(ctx);
        Ok(())
    }

    fn next_scene(&mut self, ctx: &mut Context) -> Option<Box<Scene>> {
        if self.player.is_dead() {
            Some(Box::new(StageSelectScene::new(ctx).ok()?))
        } else {
            None
        }
    }

}
