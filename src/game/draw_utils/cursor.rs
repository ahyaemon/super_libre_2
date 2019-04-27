use ggez::Context;
use ggez::GameResult;
use ggez::graphics;
use ggez::graphics::Text;
use ggez::graphics::TextFragment;
use ggez::graphics::DrawParam;
use ggez::graphics::Font;
use ggez::graphics::Scale;
use ggez::graphics::Align;

use cgmath::Point2;

use std::f32;


pub struct Cursor {
    pub position: Point2<f32>,
    pub interval: f32,
    pub item_count: i8,
    pub item_index: i8,
    text: Text,
}

impl Cursor {

    pub fn new(
        position: Point2<f32>,
        interval: f32,
        item_count: i8,
    ) -> Cursor {
        let mut text = Text::new(
            TextFragment {
                text: "->".to_string(),
                color: Some(graphics::WHITE),
                font: Some(Font::default()),
                scale: Some(Scale::uniform(60.0)),
            }
        );
        text.set_bounds(Point2::new(100.0, f32::INFINITY), Align::Left);
        Cursor {
            position,
            interval,
            item_count,
            item_index: 0,
            text,
        }
    }

    pub fn up(&mut self) {
        self.item_index = (self.item_index + self.item_count - 1) % self.item_count;
    }

    pub fn down(&mut self) {
        self.item_index = (self.item_index + 1) % self.item_count;
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        let position = Point2::new(
            self.position.x,
            self.position.y + (self.item_index as f32) * self.interval,
        );
        graphics::queue_text(ctx, &self.text, position, None);
        graphics::draw_queued_text(ctx, DrawParam::default())?;
        Ok(())
    }

}
