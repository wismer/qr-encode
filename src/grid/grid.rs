extern crate image as image_lib;

use grid::cell::{Cell, CellRef};
use grid::traverse::{Point, Direction};
use std::collections::HashMap;
use grid::image::{create_qr_image};

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
    pub current_pt: Point,
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
    pub fn skip_bridge(&self, pt: Point) -> Point {
        Point { x: pt.x - 1, y: pt.y }
    }

    pub fn get_mut_cell(&mut self, point: &Point) -> Option<&mut Cell> {
        match self.rows.get_mut(point.x) {
            Some(row) => row.cells.get_mut(point.y),
            None => None
        }
    }

    pub fn is_valid_path(&mut self, point: Option<Point>) -> bool {
        if point.is_none() {
            return false
        }

        let pt = point.unwrap();
        // println!("IS VALID {} {}", pt.x, pt.y);

        match self.rows.get(pt.x).unwrap().cells.get(pt.y) {
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

    pub fn get_cell_ref(&self, x: usize, y: usize) -> CellRef {
        let cell_ref = match self.rows.get(x) {
            Some(row) => row.cells.get(y),
            None => None
        };

        if cell_ref.is_none() {
            return CellRef::None
        }

        let cell = cell_ref.unwrap();
        cell.value()
    }

    pub fn get_next_point(&self, point: Point) -> Point {
        // hashmap key -> direction enum, value -> Vector<[Option<Point>; 3]> ?
        let adjacent_points: Vec<(isize, isize)> = vec![
            (1, 1),
            (1, 0),
            (-1, 1),
            (-1, 0)
        ];

        let best_candidate = adjacent_points.into_iter().find(|&p| {
            let next_point = point >> p;
            if next_point.is_none() {
                return false
            } else {
                let pt = next_point.unwrap();
                // println!("{:?} # {p1} {p2}", next_point=pt, p1=p.0, p2=p.1);
                let cell_ref = self.get_cell_ref(pt.x, pt.y);
                match cell_ref {
                    CellRef::Free(_) => true,
                    CellRef::Bridge(_) => true,
                    _ => false
                }
            }
        });


        if best_candidate.is_some() {
            let f = best_candidate.unwrap();
            let mut next_point = (point >> f).unwrap();
            next_point = match self.get_cell_ref(next_point.x, next_point.y) {
                CellRef::Bridge(_) => {
                    let x = self.skip_bridge(next_point);
                    println!("{:?} skipped from {:?}", x, next_point);
                    x
                },
                _ => next_point
            };
            if point ^ next_point {
                (point << 1).unwrap()
            } else {
                next_point
            }
        } else {
            println!("CHECK ME OUT");
            (point << 1).unwrap()
        }
    }
}



pub fn encode_byte(grid: &mut Grid, byte: u8, last_position: (usize, usize)) -> (usize, usize) {
    let mut i = 0u8;
    let (x, y) = last_position;
    let mut point = Point { x: x, y: y };

    while i < 8 {
        let xbit = byte & (1 << i);

        grid.encode_bit(xbit == 0, point);

        println!("x: {x} y: {y}", x=point.x, y=point.y);

        point = grid.get_next_point(point);

        // get next point

        i += 1;
    }

    (point.x, point.y)
}

pub fn create_grid(size: usize, mask: u8, qr_version: u8, message: String) {
    let rows: Vec<Row> = Vec::new();
    let max = size * size;
    let mut grid = Grid {
        rows: rows,
        current_pt: Point { x: 48, y: 48 }
    };

    for i in 0..max {
        let x = i / size;
        let y = i % size;
        grid.push(x, y, size);
    }

    let mut position = (48, 48);
    for byte in message.into_bytes() {
        position = encode_byte(&mut grid, byte, position);
        println!("{:?}", "bite me");
    }
    let mut start_points: [(Point, usize, Option<char>); 9] = [
        (Point { x: 1, y: 1 }, 5, None),
        (Point { x: 1, y: 43 }, 5, None),
        (Point { x: 43, y: 1 }, 5, None),
        (Point { x: 41, y: 0 }, 8, Some('>')),
        (Point { x: 48, y: 7 }, 8, Some('+')),
        (Point { x: 0, y: 41 }, 8, Some('-')),
        (Point { x: 7, y: 0 }, 8, Some('>')),
        (Point { x: 0, y: 7 }, 8, Some('-')),
        (Point { x: 7, y: 41 }, 8, Some('>')),
    ];
    let mut points = vec![];
    for coords in (&mut start_points).into_iter() {
        //let _: () = coords;
        let (pt, blocks, operator) = *coords;

        println!("{:?}", coords);

        if operator.is_none() {
            for p in Point::square_points(pt, blocks) {
                points.push(p);
            }
        } else {
            // stupid_friggin_areas_that_also_need_to_be_white..
            points.push(pt);
            Point::line(&mut points, blocks, operator.unwrap());
        }
    }

    for point in points {
        match grid.get_mut_cell(&point) {
            Some(mut cell) => cell.is_filled = true,
            None => {}
        }
    }


    create_qr_image(grid);
}
