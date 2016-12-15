extern crate image as image_lib;

use std::fs::File;
use std::path::Path;

use grid::cell::Cell;
use grid::traverse::Point;
use grid::util::*;
use grid::image::{create_qr_image, get_pixel_points};
use self::image_lib::{
    ImageBuffer,
    Rgba,
    Pixel
};
use self::image_lib::gif::{Encoder, Frame};
use std::collections::HashMap;
use std::borrow::Cow;

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


enum GridPoint {
    Free(Point),
    Used(Point),
    Fixed,
    Bridge,
    None
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

    pub fn get_cell_ref(&self, x: usize, y: usize) -> Option<&Cell> {
        let cell_ref = match self.rows.get(x) {
            Some(row) => row.cells.get(y),
            None => None
        };

        if cell_ref.is_none() {
            return None
        }

        let cell = cell_ref.unwrap();

        if cell.is_free() {
            Some(cell)
        } else {
            None
        }
    }

    pub fn get_next_point(&self, point: Point) -> Point {
        let adjacent_points: Vec<(isize, isize)> = vec![
            (1, 1),
            (-1, 1),
            (1, 0),
            (-1, 0)
        ];

        let best_candidate = adjacent_points.into_iter().find(|&p| {
            let next_point = point >> p;
            if next_point.is_none() {
                return false
            } else {
                let pt = next_point.unwrap();
                println!("x: {x} y: {y}, from {:?}", p=p, x=pt.x, y=pt.y);

                let cell_ref = self.get_cell_ref(pt.x, pt.y);
                match cell_ref {
                    Some(_) => true,
                    None => false
                }
            }
        });


        if best_candidate.is_some() {
            let next_point = (point >> best_candidate.unwrap()).unwrap();
            if point ^ next_point {
                (point << 1).unwrap()
            } else {
                next_point
            }
        } else {
            (point << 1).unwrap()
        }
    }
}



pub fn encode_byte(grid: &mut Grid, byte: u8, last_position: (usize, usize)) -> (usize, usize, ImageBuffer<Rgba<u8>, Vec<u8>>) {
    let mut i = 7u8;
    let (x, y) = last_position;
    let mut point = Point { x: x, y: y };
    let mut img = ImageBuffer::new(49 * 20, 49 * 20);

    while i > 0 {
        let xbit = byte & (1 << i);

        grid.encode_bit(xbit == 0, point);
        point = grid.get_next_point(point);

        // get next point

        i -= 1;
    }

    for row in &grid.rows {
        for cell in &row.cells {
            for pixel in get_pixel_points(&cell) {
                let (x, y, color) = pixel;
                let rgb = Rgba { data: [color.r, color.g, color.b, 1] };
                img.put_pixel(x, y, rgb);
            }
        }
    }


    (point.x, point.y, img)
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
    let (cx, cy) = (48, 48);
    let mut fout = File::create(&Path::new("qr.gif")).unwrap();
    for byte in message.into_bytes() {
        let (cx, cy, buffer) = encode_byte(&mut grid, byte, (cx, cy));
        let mut encoder = Encoder::new(&mut fout);
        let mut frame = Frame::default();
        frame.buffer = Cow::Borrowed(&*buffer);
        frame.height = 49 * 20;
        frame.width = 49 * 20;
        let result = encoder.encode(frame);
        match result {
            Ok(_) => println!("IT WORKED"),
            Err(e) => println!("{:?}", e)
        }
    }

    create_qr_image(grid);
}
