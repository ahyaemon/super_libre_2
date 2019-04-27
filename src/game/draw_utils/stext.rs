use ggez::Context;
use ggez::GameResult;
use ggez::graphics;
use ggez::graphics::Text;
use ggez::graphics::DrawParam;

use cgmath::Point2;


pub struct SText{
    pub text: Text,
    pub position: Point2<f32>,
}

impl SText {

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        graphics::queue_text(ctx, &self.text, self.position, None);
        graphics::draw_queued_text(ctx, DrawParam::default())?;
        Ok(())
    }

}
