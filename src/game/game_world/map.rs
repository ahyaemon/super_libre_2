mod cell;
mod row;

use ggez::GameResult;
use ggez::GameError;
use ggez::Context;

use super::sprite::Sprite;
use self::row::Row;
use csv::Reader;
use std::fs::File;
use crate::game::game_world::map::cell::Cell;
use super::sprite::background::block::Block;
use cgmath::Point2;


pub struct Map {
    rows: Vec<Row>
}

impl Map {

    pub fn from_filename(ctx: &mut Context, filename: &str) -> GameResult<Map> {
        let mut reader = create_reader(filename)?;
        let mut record = csv::ByteRecord::new();
        let mut rows = vec![];
        let mut irow = 0i8;
        while reader.read_byte_record(&mut record).map_err(|_e| GameError::FilesystemError("invalid csv file".to_string()))? {
            let mut row = vec![];
            for i in 0..record.len() {
                let cell = Cell{ sprite: create_sprite(ctx, &record[i], irow, i as i8)? };
                row.push(cell);
            }
            rows.push(Row{ cells: row });
            irow += 1;
        }
        Ok(Map{rows})
    }

    pub fn update(&mut self, ctx: &mut Context) {
        for row in &mut self.rows {
            for cell in &mut row.cells {
                if let Some(sprite) = &mut cell.sprite {
                    sprite.update(ctx);
                }
            }
        }
    }

    pub fn draw(&self, ctx: &mut Context) {
        for row in &self.rows {
            for cell in &row.cells {
                if let Some(sprite) = &cell.sprite {
                    sprite.draw(ctx);
                }
            }
        }
    }

    /// 指定座標のセルにスプライトがある場合、その参照を返す
    pub fn get_sprite(&self, irow: usize, icol: usize) -> Option<&Sprite> {
        let row = &self.rows[irow];
        let cell = row.get_cell(icol);
        if let Some(sprite_box) = &cell.sprite {
            Some(sprite_box.as_ref())
        } else {
            None
        }
    }

}

fn create_reader(filename: &str) -> GameResult<Reader<File>> {
    csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .from_path(filename)
        .map_err(|_e| GameError::FilesystemError("invalid csv file".to_string()))
}

fn create_sprite(ctx: &mut Context, u: &[u8], irow: i8, icol: i8) -> GameResult<Option<Box<Sprite>>> {
    match u {
        b"1" => {
            let x = (icol as f32) * 32.0;
            let y = (irow as f32) * 32.0;
            let position = Point2::new(x, y);
            Ok(Some(
                Box::new(Block::new(ctx, position)?)
            ))
        }
        _ => {
            Ok(None)
        }
    }
}
