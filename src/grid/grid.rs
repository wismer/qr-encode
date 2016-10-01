use grid::message::{FormatInfo, ErrorCorrectionLevel};
use std::collections::HashMap;
use grid::traverse::Point;
use grid::bit::{Bit};
use grid::util::*;


pub enum QRSection {
    Fixed,
    FixedBridge,
    Format,
    ContentBody,
    EncType,
    MetaData
}

struct Routes {
    leftward: Option<usize>,
    rightward: Option<usize>,
    forward: Option<usize>,
    backward: Option<usize>,
    upper_right: Option<usize>,
    upper_left: Option<usize>,
    lower_left: Option<usize>,
    lower_right: Option<usize>
}

pub struct QRGrid {
    size: usize,
    pub bits: Vec<Bit>,
    index: usize,
    format_info: FormatInfo
}

impl Routes {
    fn next(&self) -> usize {
        if self.leftward.is_some() && self.forward.is_some() && self.upper_right.is_some() {
            self.upper_right.unwrap()
        } else if self.leftward.is_some() {
            self.leftward.unwrap()
        } else {
            0
        }
    }

    fn available_paths(&self) -> usize {
        let mut count = 0;

        if self.leftward.is_some() {
            count += 1;
        }

        if self.rightward.is_some() {
            count += 1;
        }

        if self.forward.is_some() {
            count += 1;
        }

        if self.backward.is_some() {
            count += 1;
        }

        count
    }
}

// pub enum Cell {
//     Fixed(usize, usize),
//     Format(u8),
//     Content(Point)
// }

pub struct Cell {
    is_fixed: bool,
    is_bridge: bool,
    is_empty: bool,
    is_bit: bool,
    is_filled: bool,
    x: usize,
    y: usize,
}


pub struct Row {
    cells: Vec<Cell>
}

pub struct Grid {
    rows: Vec<Row>
}


impl Grid {
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
                    let cell = create_cell(x, y, size);
                    // and append it to that particular row being built
                    row.cells.push(cell);
                } else {
                    // if the current row is full, then a new one needs to be created
                    // and then bound to `new_row`
                    make_row = row_count <= 49;
                }
            },
            None => {
                make_row = true;
                // there are no rows, so I have to make one!
            }
        }

        if make_row && row_count < 49 {
            self.rows.push(Row { cells: Vec::new() });
        }

        println!("size: {}", self.rows.len());
    }

    pub fn size_of_grid(&self) -> usize {

        for row in &self.rows {
            println!("{}, actual length: {}", row.cells.len() == 49, row.cells.len());
        }
        self.rows.len()
    }
}

fn create_cell(x: usize, y: usize, size: usize) -> Cell {
    if is_format_area(x, y, size) {
        Cell {
            is_fixed: true,
            is_bridge: false,
            is_empty: true,
            is_bit: false,
            is_filled: false,
            x: y,
            y: x
        }
    } else if is_fixed_area(x, y, size) {
        Cell {
            is_fixed: true,
            is_bridge: is_bridge_area(x, y, size),
            is_empty: true,
            is_bit: false,
            is_filled: false,
            x: y,
            y: x
        }
    } else {
        Cell {
            is_fixed: true,
            is_bridge: false,
            is_empty: true,
            is_bit: false,
            is_filled: false,
            x: y,
            y: x
        }
    }
}

pub fn create_grid(size: usize, mask: u8, qr_version: u8) -> Grid {
    let cells: Vec<Cell> = Vec::new();
    let row = Row { cells: cells };
    let rows: Vec<Row> = Vec::new();
    let mut grid = Grid { rows: rows };
    for i in 0..(size * size) {
        let x = i / size;
        let y = i % size;
        println!("x={} y={}", x, y);
        grid.push(x, y, size);
    }
    grid
}

pub fn encode_byte(byte: u8, cells: &mut Vec<Cell>, size: usize) {
    let mut i = 8;
    let mut index = size - 1;
    while i >= 0 {
        // let ref mut point = get_current_cell(index, cells).unwrap();
        let cell = cells.get_mut(index).unwrap();
        {
            let xbit = byte & (1 << i);
            if xbit == 1 {
                // cell = Cell::Content(Point { x: 10, y: 10, is_bit: true, is_corner: false, is_edge: false });
            }
            i -= 1;
        }
    }
}


pub fn get_current_cell(idx: usize, cells: &mut Vec<Cell>) -> Option<&Point> {
    match *cells.get_mut(idx).unwrap() {
        // Cell::Content(ref mut point) => Some(point),
        _ => None
    }
}
