use ggez::Context;
use ggez::GameResult;
use ggez::audio::Source;
use ggez::audio::SoundSource;

pub struct StageSelectSound {
    pub bgm: Source,
    pub se_hyoushigi: Source,
}

impl StageSelectSound {

    pub fn new(ctx: &mut Context) -> GameResult<StageSelectSound> {
        let mut sound = StageSelectSound {
            bgm: Source::new(ctx, "/sound/bgm/chronicle.mp3")?,
            se_hyoushigi: Source::new(ctx, "/sound/se/hyoushigi1.mp3")?,
        };
        sound.bgm.set_repeat(true);
        Ok(sound)
    }

}