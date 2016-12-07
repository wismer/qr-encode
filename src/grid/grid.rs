use grid::cell::Cell;
use grid::traverse::Point;
use grid::util::*;
use grid::image::{create_qr_image};
use std::collections::HashMap;

pub enum QRSection {
    Fixed,
    FixedBridge,
    Format,
    ContentBody,
    EncType,
    MetaData
}

pub struct Row {
    pub cells: Vec<Cell>
}

pub struct Grid {
    pub rows: Vec<Row>
}


impl<'a> Grid {
    fn push(&mut self, x: usize, y: usize, size: usize) {
        let mut make_row = false;
        let row_count = self.rows.len();
        // grab the last row in the vector
        match self.rows.last_mut() {
            // if there exists a row already...
            Some(row) => {
                // check to see if it is not full (being 49 for now)
                if row.cells.len() < 49 {
                    // if it can still accept cells, then create one with the given coordinate
                    let cell = Cell::new(x, y, size);
                    // and append it to that particular row being built
                    row.cells.push(cell);
                } else {
                    // if the current row is full, then a new one needs to be created
                    // and then bound to `new_row`
                    make_row = row_count < 49;
                }
            },
            None => {
                make_row = true;
                // there are no rows, so I have to make one!
            }
        }

        if make_row && row_count < 49 {
            let mut row = Row { cells: Vec::new() };
            let cell = Cell::new(x, y, size);
            row.cells.push(cell);
            self.rows.push(row);
        }
    }

    // PUBLIC
    pub fn get_mut_cell(&mut self, point: &Point) -> Option<&mut Cell> {
        match self.rows.get_mut(point.x) {
            Some(row) => row.cells.get_mut(point.y),
            None => None
        }
    }

    pub fn is_valid_path(&mut self, point: Point) -> bool {
        match self.rows.get(point.x).unwrap().cells.get(point.y) {
            Some(cell) => cell.is_free(),
            None => false
        }
    }

    pub fn encode_bit(&mut self, is_bit: bool, point: Point) {
        let mut cell = self.get_mut_cell(&point).unwrap();
        cell.is_bit = is_bit;
        cell.is_empty = false;
        cell.is_filled = true;
    }
}

pub fn encode_byte(grid: &mut Grid, byte: u8) {

}

pub fn create_grid(size: usize, mask: u8, qr_version: u8, message: String) {
    let cells: Vec<Cell> = Vec::new();
    let rows: Vec<Row> = Vec::new();
    let max = (size * size);
    let mut grid = Grid { rows: rows };

    for i in 0..max {
        let x = i / size;
        let y = i % size;
        grid.push(x, y, size);
    }

    for byte in message.into_bytes() {
        encode_byte(&mut grid, byte);
    }

    create_qr_image(grid);
}
