use ggez::{
    Context,
    GameResult,
    audio::{
        Source, SoundSource
    },
};


pub struct TitleSound {
    bgm_juvenile: Source,
    se_hyoushigi: Source,
}

impl TitleSound {

    pub fn new(ctx: &mut Context) -> GameResult<TitleSound> {
        let mut bgm_juvenile = Source::new(ctx, "/sound/bgm/juvenile.mp3")?;
        bgm_juvenile.set_repeat(true);
        let se_hyoushigi = Source::new(ctx, "/sound/se/hyoushigi1.mp3")?;

        Ok(
            TitleSound {
                bgm_juvenile,
                se_hyoushigi,
            }
        )
    }

    pub fn play_bgm_juvenile(&mut self) {
        let _ = self.bgm_juvenile.play_detached();
    }

    pub fn play_se_hyoushigi(&mut self) {
        let _ = self.se_hyoushigi.play();
    }

}