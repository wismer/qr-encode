use grid::message::{FormatInfo, ErrorCorrectionLevel};
use grid::cell::Cell;
use grid::traverse::Point;
use grid::util::*;


pub enum QRSection {
    Fixed,
    FixedBridge,
    Format,
    ContentBody,
    EncType,
    MetaData
}

pub enum Direction {
    Top(Point),
    Bottom(Point),
    Left(Point),
    Right(Point),
    None
}

pub struct Row {
    pub cells: Vec<Cell>
}

pub struct Grid {
    pub rows: Vec<Row>
}


impl<'a> Grid {
    pub fn update_cell_paths(&mut self, point: Point) {
        let mut paths = 0;
        for adjacent in Point::generate_adjacent_points(point) {
            if self.is_valid_path(adjacent) {
                paths += 1;
            } else if paths > 0 {
                paths -= 1;
            }
        }

        let mut row = self.rows.get_mut(point.x).unwrap();
        match row.cells.get_mut(point.y) {
            Some(cell) => {
                cell.paths = paths;
            },
            None => {}
        }
    }

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

    fn fetch_cell(&self, pt: Option<Point>) -> Option<Point> {
        // if point is out of bounds, it will return None.
        if pt.is_none() {
            panic!("This should not trigger");
        }
        // otherwise, the point is valid, but the cell itself may not be a valid _choice_
        let point = pt.unwrap();
        match self.rows.get(point.x).unwrap().cells.get(point.y) {
            Some(cell) => {
                if cell.is_free() {
                    Some(Point { x: cell.x, y: cell.y })
                } else {
                    None
                }
            },
            None => None
        }
    }

    // PUBLIC

    pub fn get_next_valid_point(&self, current_point: &Point) -> Option<Point> {
        let mut done = false;
        let mut next_point = *current_point >> 1;
        if next_point.is_none() {
            // rightmost edge. Alignment is to grow upwards.
            return Some((*current_point << 1).unwrap())
        }

        next_point.unwrap() - 1
    }

    pub fn encode_bit(&mut self, is_bit: bool, point: Point) {
        let mut cell = self.get_mut_cell(&point).unwrap();
        cell.is_bit = is_bit;
        cell.is_empty = false;
        cell.is_filled = true;
    }

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

pub fn encode_byte(byte: u8, grid: &mut Grid, index: &mut usize, step: usize) {
    let xbit = byte & (1 << step);

    // let mut i = 7;
    // while i >= 0 {
    //
    //     let xbit = byte & (1 << i);
    //     // grid.set_cell(*index, xbit == 0);
    //     let (x, y) = (*index / 49, *index % 49);
    //
    //     {
    //         let mut cell = get_mut_cell(&mut grid, x, y);
    //         match cell {
    //             Some(c) => c.is_bit = xbit == 0 && !c.is_fixed,
    //             None => {}
    //         }
    //     }
    //
    //     {
    //
    //     }
    //     // index -= pick_next_index(index);
    //     i -= 1;
    //
    //     *index -= 1;
    // }
}

fn get_mut_cell<'a>(grid: &'a mut Grid, x: usize, y: usize) -> Option<&'a mut Cell> {
    match grid.rows.get_mut(x) {
        Some(row) => row.cells.get_mut(y),
        None => None
    }
}
