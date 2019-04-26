use ggez::{
    Context,
    GameResult,
    audio::{
        Source, SoundSource
    },
};


pub struct TitleSound {
    pub bgm: Source,
    pub se_hyoushigi: Source,
}

impl TitleSound {

    pub fn new(ctx: &mut Context) -> GameResult<TitleSound> {
        let mut sound = TitleSound {
            bgm: Source::new(ctx, "/sound/bgm/welcome_to_the_super_libre_2.mp3")?,
            se_hyoushigi: Source::new(ctx, "/sound/se/hyoushigi1.mp3")?,
        };
        sound.bgm.set_repeat(true);
        Ok(sound)
    }

}