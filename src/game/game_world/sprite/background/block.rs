use crate::game::game_world::sprite::Sprite;
use ggez::Context;
use cgmath::Point2;
use ggez::graphics::Image;
use ggez::error::GameResult;
use ggez::graphics;

// TODO 全てのブロックに対して Image を new するのは無駄
pub struct Block {
    image: Image,
    position: Point2<f32>,
}

impl Block {

    pub fn new(ctx: &mut Context, position: Point2<f32>) -> GameResult<Block> {
        Ok(
            Block{
                image: Image::new(ctx, "/img/background/block/ground.png")?,
                position,
            }
        )
    }

}

impl Sprite for Block {

    fn update(&mut self, _ctx: &mut Context) {
    }

    fn draw(&self, ctx: &mut Context) {
        let _ = graphics::draw(ctx, &self.image, (self.position,));
    }

    fn get_top(&self) -> f32 {
        self.position.y
    }

    fn get_right(&self) -> f32 {
        self.position.x + 32.0
    }

    fn get_bottom(&self) -> f32 {
        self.position.y + 32.0
    }

    fn get_left(&self) -> f32 {
        self.position.x
    }

}
