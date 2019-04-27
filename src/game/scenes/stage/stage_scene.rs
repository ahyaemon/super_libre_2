use ggez::Context;
use ggez::GameResult;
use ggez::audio::SoundSource;

use crate::game::scenes::Scene;
use crate::game::input::Input;
use crate::game::game_world::sprites::Player;

use super::stage_sound::StageSound;


pub struct StageScene {
    player: Player,
    sound: StageSound,
}

impl StageScene {

    pub fn new(ctx: &mut Context) -> GameResult<StageScene> {
        let mut scene = StageScene {
            player: Player::new(ctx)?,
            sound: StageSound::new(ctx)?,
        };
        let _ = scene.sound.bgm.play();
        Ok(scene)
    }

}

impl Scene for StageScene {

    fn update(&mut self, ctx: &mut Context, input: &Input) -> GameResult {
        self.player.input(ctx, input);
        self.player.update(ctx);
        Ok(())
    }

    fn draw(&self, ctx: &mut Context) -> GameResult {
        self.player.draw(ctx);
        Ok(())
    }

    fn next_scene(&mut self, _ctx: &mut Context) -> Option<Box<Scene>> {
        None
    }

}
