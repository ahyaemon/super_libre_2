// TODO: パッド入力を表す　Input と、各画面に対応した Command が必要？

use ggez::{
    event::{
        KeyCode,
    },
};

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum InputType {
    Up,
    Right,
    Down,
    Left,
    Start,
//    Select,
}

impl InputType {

    fn from(key_code: KeyCode) -> Option<InputType> {
        match key_code {
            KeyCode::A => { Some(InputType::Left) }
            KeyCode::S => { Some(InputType::Down) }
            KeyCode::D => { Some(InputType::Right) }
            KeyCode::W => { Some(InputType::Up) }

            KeyCode::Left => { Some(InputType::Left) }
            KeyCode::Down => { Some(InputType::Down) }
            KeyCode::Right => { Some(InputType::Right) }
            KeyCode::Up => { Some(InputType::Up) }

            KeyCode::Return => { Some(InputType::Start) }

            _ => { None }
        }
    }

}

pub struct Input {
    pub pressed: Option<InputType>,
    pub pressing: Vec<InputType>,
    pub released: Option<InputType>,
}

impl Input {

    pub fn new() -> Input {
        Input {
            pressed: None,
            pressing: Vec::new(),
            released: None,
        }
    }

    pub fn key_down(&mut self, key_code: KeyCode) {
        // キー入力からゲーム入力に変換
        let input_type = InputType::from(key_code);

        // 押された判定に設定（pressingに存在しない場合）
        if let Some(it) = input_type {
            if !self.pressing.contains(&it) {
                self.pressed = input_type;
            }
        }

        // 押されっぱなし判定に追加（存在しない場合）
        if let Some(it) = input_type {
            if !self.pressing.contains(&it) {
                self.pressing.push(it);
            }
        }
    }

    pub fn key_up(&mut self, key_code: KeyCode) {
        // キー入力からゲーム入力に変換
        let input_type = InputType::from(key_code);

        // キーリリース判定に設定
        self.released = input_type;

        // 押されっぱなし判定から削除
        if let Some(it) = input_type {
            if let Some(index) = self.pressing.iter().position(|x| x == &it) {
                self.pressing.remove(index);
            }
        }
    }

    pub fn clear(&mut self) {
        self.pressed = None;
        self.released = None;
    }

    // デバッグで確認する用
    #[allow(dead_code)]
    pub fn show(&self) {
        println!("INPUT ---");
        // pressed
        print!("pressed: ");
        if let Some(input_type) = &self.pressed {
            print!("{:?}", input_type);
        }
        println!();

        // released
        print!("released: ");
        if let Some(input_type) = &self.released {
            print!("{:?}", input_type);
        }
        println!();

        // pressing
        print!("pressing: ");
        for input_type in &self.pressing {
            print!("{:?}", input_type);
            print!(", ");
        }
        println!();
    }

}