use ggez::Context;
use ggez::GameResult;
use ggez::graphics::Text;
use ggez::graphics::Scale;
use ggez::graphics::TextFragment;
use ggez::graphics::Color;
use ggez::graphics::Font;
use ggez::graphics::Align;
use ggez::audio::SoundSource;

use cgmath::Point2;

use std::f32;

use crate::game::scenes::Scene;
use crate::game::scenes::StageSelectScene;
use crate::game::draw_utils::SText;
use crate::game::draw_utils::Cursor;
use crate::game::input::Input;
use crate::game::input::InputType;

use super::title_sound::TitleSound;


pub struct TitleScene {
    title_text: SText,
    start_text: SText,
    how_text: SText,
    config_text: SText,
    cursor: Cursor,
    sound: TitleSound,
    selected: bool,
}

impl TitleScene {

    pub fn new(ctx: &mut Context) -> GameResult<TitleScene> {
        let white = Some(Color::new(1.0, 1.0, 1.0, 1.0));
        let default_font = Some(Font::default());
        let title_scale = 100.0;
        let item_scale = 60.0;

        let menu_x = 290.0;
        let menu_y = 280.0;
        let menu_d = 80.0;

        // title
        let mut title = Text::new(
            TextFragment {
                text: "SUPER LIBRE 2".to_string(),
                color: white,
                font: default_font,
                scale: Some(Scale::uniform(title_scale)),
            }
        );
        title.set_bounds(Point2::new(800.0, f32::INFINITY), Align::Center);
        let stitle = SText {
            text: title,
            position: Point2::new(0.0, 80.0),
        };

        // start
        let mut start = Text::new(
            TextFragment {
                text: "START".to_string(),
                color: white,
                font: default_font,
                scale: Some(Scale::uniform(item_scale)),
            }
        );
        start.set_bounds(Point2::new(400.0, f32::INFINITY), Align::Left);
        let sstart = SText {
            text: start,
            position: Point2::new(menu_x, menu_y),
        };

        // how
        let mut how = Text::new(
            TextFragment {
                text: "HOW".to_string(),
                color: white,
                font: default_font,
                scale: Some(Scale::uniform(item_scale)),
            }
        );
        how.set_bounds(Point2::new(400.0, f32::INFINITY), Align::Left);
        let show = SText {
            text: how,
            position: Point2::new(menu_x, menu_y + menu_d),
        };

        // config
        let mut config = Text::new(
            TextFragment {
                text: "CONFIG".to_string(),
                color: white,
                font: default_font,
                scale: Some(Scale::uniform(item_scale)),
            }
        );
        config.set_bounds(Point2::new(400.0, f32::INFINITY), Align::Left);
        let sconfig = SText {
            text: config,
            position: Point2::new(menu_x, menu_y + 2.0 * menu_d),
        };

        // config
        let cursor = Cursor::new(Point2::new(menu_x - 80.0, menu_y), 80.0, 3);

        let mut sound = TitleSound::new(ctx)?;
        let _ = sound.bgm.play();

        let title_scene = TitleScene {
            title_text: stitle,
            start_text: sstart,
            how_text: show,
            config_text: sconfig,
            cursor,
            sound,
            selected: false,
        };
        Ok(title_scene)
    }

}

impl Scene for TitleScene {

    fn update(&mut self, _ctx: &mut Context, input: &Input) -> GameResult {
        if let Some(input_type) = &input.pressed {
            match input_type {
                InputType::Up => {
                    let _ = self.sound.se_hyoushigi.play();
                    self.cursor.up();
                }
                InputType::Down => {
                    let _ = self.sound.se_hyoushigi.play();
                    self.cursor.down();
                }
                InputType::Start => {
                    let _ = self.sound.se_hyoushigi.play();
                    self.selected = true;
                }
                _ => {}
            }
        };
        Ok(())
    }

    fn draw(&self, ctx: &mut Context) -> GameResult {
        self.title_text.draw(ctx)?;
        self.start_text.draw(ctx)?;
        self.how_text.draw(ctx)?;
        self.config_text.draw(ctx)?;
        self.cursor.draw(ctx)?;
        Ok(())
    }

    fn next_scene(&mut self, ctx: &mut Context) -> Option<Box<Scene>> {
        if self.selected {
            Some(
                match self.cursor.item_index {
                    0 => {
                        self.sound.bgm.stop();
                        Box::new(StageSelectScene::new(ctx).ok()?)
                    }
                    _ => {
                        Box::new(TitleScene::new(ctx).ok()?)
                    }
                }
            )
        } else {
            None
        }
    }

}
