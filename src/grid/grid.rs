use grid::message::{FormatInfo, ErrorCorrectionLevel};
use std::collections::HashMap;
use grid::traverse::Coord;
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


pub struct Cell {
    pub is_fixed: bool,
    pub is_bridge: bool,
    pub is_empty: bool,
    pub is_bit: bool,
    pub is_filled: bool,
    pub is_format: bool,
    pub x: usize,
    pub y: usize,
    pub paths: usize
}

impl Cell {
    pub fn is_free(&self) -> bool {
        !self.is_fixed && self.is_empty && !self.is_format && !self.is_bridge
    }
}

pub struct Row {
    pub cells: Vec<Cell>
}

pub struct Grid {
    pub rows: Vec<Row>
}


impl Grid {
    fn set_cell(&mut self, index: usize, is_bit: bool)  {
        let (x, y) = ((index / 49), index % 49);
        let mut row = self.rows.get_mut(x).unwrap();
        let mut cell = row.cells.get_mut(y).unwrap();
        cell.is_bit = is_bit;
    }

    fn set_number_pathways_for_cell(&mut self, x: usize, y: usize, count: usize) {
        match self.rows.get_mut(x) {
            Some(row) => {
                match row.cells.get_mut(y) {
                    Some(cell) => cell.paths += count,
                    None => {}
                }
            },
            None => {}
        }
    }

    pub fn get_neighboring_cells(&mut self, index: usize) {
        let point: Coord = Coord { x: index / 49, y: index % 49 };
        let size = 49 * 49;
        let shift_operators = "< > - +";
        let mut pathways_for_point = 0;
        for c in shift_operators.chars() {
            let pt: Option<(usize, usize)> = match c {
                '>' => point << 1,
                '<' => point >> 1,
                '+' => point +  1,
                '-' => point -  1,
                 _  => None
            };



            if pt.is_some() {
                let (x, y) = pt.unwrap();
                println!("X: {}, Y: {}", x, y);

                let row = self.rows.get(x);
                match row {
                    Some(row) => {
                        let cell = row.cells.get(y);
                        match cell {
                            Some(cell) => {
                                if cell.is_free() {
                                    pathways_for_point += 1;
                                }
                            },
                            None => {}
                        }
                    },
                    None => {}
                }
            }
        }

        self.set_number_pathways_for_cell(point.x, point.y, pathways_for_point);
    }

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
            let cell = create_cell(x, y, size);
            row.cells.push(cell);
            self.rows.push(row);
        }
    }

    // PUBLIC

    pub fn size_of_grid(&self) -> usize {
        let mut x = 1;
        let mut total = 0;
        for row in &self.rows {
            x += 1;
            total += row.cells.len();
        }
        self.rows.len()
    }
}

fn create_cell(x: usize, y: usize, size: usize) -> Cell {
    Cell {
        is_fixed: is_fixed_area(x, y, size - 1),
        is_bridge: is_bridge_area(x, y, size - 1),
        is_empty: true,
        is_bit: false,
        is_filled: false,
        is_format: is_format_area(x, y, size - 1),
        x: y,
        y: x,
        paths: 0
    }
}

pub fn create_grid(size: usize, mask: u8, qr_version: u8) -> Grid {
    let cells: Vec<Cell> = Vec::new();
    let row = Row { cells: cells };
    let rows: Vec<Row> = Vec::new();
    let max = (size * size);
    let mut grid = Grid { rows: rows };
    for i in 0..max {
        let x = i / size;
        let y = i % size;
        grid.push(x, y, size);
    }
    grid
}
//
// fn update_adjacent_cells(grid: &mut Grid, current_cell: &Cell) -> Cell {
//
// }

pub fn encode_byte(byte: u8, grid: &mut Grid, index: &mut usize) {
    let mut i = 7;
    while i >= 0 {
        {
            let xbit = byte & (1 << i);
            grid.set_cell(*index, xbit == 0);
            // index -= pick_next_index(index);
            i -= 1;

            *index -= 1;
        }
    }
}
