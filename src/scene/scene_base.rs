use ggez::{
    Context,
    GameResult,
};

pub trait Scene {
    fn update(&mut self) -> GameResult;
    fn draw(&self, ctx: &mut Context) -> GameResult;
}
