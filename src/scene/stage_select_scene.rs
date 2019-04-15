use crate::scene::Scene;
use ggez::{
    Context,
    GameResult,
    graphics::{
        Text,
        Scale,
        TextFragment,
        Color,
        Font,
        Align,
    },
};
use cgmath::Point2;
use std::f32;
use crate::stext::SText;
use crate::cursor::Cursor;
use crate::input::{
    Input,
    InputType,
};

