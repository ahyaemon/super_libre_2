use super::scene_base::Scene;
use ggez::{
    Context,
    GameResult,
    graphics::{
        self,
        Text,
        DrawParam,
        Scale,
        TextFragment,
        Color,
        Font,
        Align,
    },
};
use cgmath::Point2;
use std::f32;

pub struct TitleScene {
    title: Text,
    title_position: Point2<f32>,
    subtitle: Text,
    subtitle_position: Point2<f32>,
}

impl TitleScene {

    pub fn new(ctx: &mut Context) -> GameResult<TitleScene> {
        let mut title = Text::new(
            TextFragment {
                text: "SUPER LIBRE 2".to_string(),
                color: Some(Color::new(1.0, 1.0, 1.0, 1.0)),
                font: Some(Font::default()),
                scale: Some(Scale::uniform(100.0)),
            }
        );
        title.set_bounds(Point2::new(800.0, f32::INFINITY), Align::Center);
        let title_position = Point2::new(0.0, 0.0);

        let subtitle = Text::new(
            format!("SUB TITLE")
        );
        let subtitle_position = Point2::new(0.0, 100.0);

        let title_scene = TitleScene {
            title,
            title_position,
            subtitle,
            subtitle_position,
        };
        Ok(title_scene)
    }

}

impl Scene for TitleScene {

    fn update(&mut self) -> GameResult {
        Ok(())
    }

    fn draw(&self, ctx: &mut Context) -> GameResult {
        graphics::queue_text(ctx, &self.title, self.title_position, None);
        graphics::queue_text(ctx, &self.subtitle, self.subtitle_position, None);
        graphics::draw_queued_text(ctx, DrawParam::default())?;
        Ok(())
    }

}