use super::cell::Cell;

pub struct Row {
    pub cells: Vec<Cell>
}

impl Row {

    pub fn get_cell(&self, icol: usize) -> &Cell {
        &self.cells[icol]
    }

}
