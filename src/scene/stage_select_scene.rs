use ggez::Context;
use ggez::GameResult;
use ggez::graphics::Text;
use ggez::graphics::Scale;
use ggez::graphics::TextFragment;
use ggez::graphics::Color;
use ggez::graphics::Font;
use ggez::graphics::Align;

use cgmath::Point2;
use std::f32;

use crate::scene::Scene;
use crate::scene::stage_select_sound::StageSelectSound;
use crate::stext::SText;
use crate::input::{Input, InputType};
use crate::cursor::Cursor;
use crate::scene::stage_scene::StageScene;

pub struct StageSelectScene {
    cursor: Cursor,
    stages: Vec<SText>,
    selected: bool,
}

impl StageSelectScene {

    pub fn new(_ctx: &mut Context) -> GameResult<StageSelectScene> {
        let cursor = Cursor::new(Point2::new(100.0, 100.0), 100.0, 3);
        let mut stages = vec![];
        for i in 0..3 {
            let mut text = Text::new(TextFragment {
                text: format!("STAGE{}", i),
                color: Some(Color::new(1.0, 1.0, 1.0, 1.0)),
                font: Some(Font::default()),
                scale: Some(Scale::uniform(60.0)),
            });
            text.set_bounds(Point2::new(400.0, f32::INFINITY), Align::Left);
            stages.push(
                SText {
                    text,
                    position: Point2::new(200.0, 100.0 + (i as f32) * 100.0),
                }
            );
        }

        Ok(StageSelectScene {
            cursor,
            stages,
            selected: false,
        })
    }

}

impl Scene for StageSelectScene {
    fn update(&mut self, _ctx: &mut Context, input: &Input) -> GameResult {
        if let Some(input_type) = &input.pressed {
            match input_type {
                InputType::Up => {
//                    let _ = self.sound.se_hyoushigi.play();
                    self.cursor.up();
                }
                InputType::Down => {
//                    let _ = self.sound.se_hyoushigi.play();
                    self.cursor.down();
                }
                InputType::Start => {
//                    let _ = self.sound.se_hyoushigi.play();
                    self.selected = true;
                }
                _ => {}
            }
        };
        Ok(())
    }

    fn draw(&self, ctx: &mut Context) -> GameResult {
        self.cursor.draw(ctx)?;
        for stage in &self.stages {
            stage.draw(ctx)?;
        }
        Ok(())
    }

    fn next_scene(&mut self, ctx: &mut Context) -> Option<Box<Scene>> {
        if self.selected {
            Some(Box::new(StageScene::new(ctx).ok()?))
        } else {
            None
        }
    }

}
