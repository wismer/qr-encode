
pub mod grid;
extern crate image as image_lib;
use grid::qr_opts::QROptions;

use std::env::{args_os};
use std::ffi::{OsStr};

use std::fs::File;
use std::path::Path;

use self::image_lib::{
    ImageBuffer,
    Rgba
};

pub fn get_pixel_points(cell: &Cell) -> Vec<(u32, u32, Color)> {
    let i = (cell.point.0 * 20) as u32;
    let j = (cell.point.1 * 20) as u32;
    let mut pixels: Vec<(u32, u32, Color)> = vec![];
    for row in i..(i + 20) {
        let x_border = match row % 20 {
            2...18 => false,
            _ => true
        };
        for col in j..(j + 20) {
            let y_border = match col % 20 {
                2...18 => false,
                _ => true
            };
            let color: Color;
            // let shader = match cell.paths {
            //     0 => 30,
            //     1 => 45,
            //     3 => 90,
            //     4 => 120,
            //     _ => 0
            // };
            // if x_border || y_border {
            //     color = Color { r: 125, b: 125, g: 125 };
            // }

            pixels.push((col, row, Color { g: cell.color.g, b: cell.color.b, r: cell.color.r }));
        }
    }

    pixels
}

fn square_count(version: usize) -> usize {
    (((version - 1) * 4) + 21)
}

fn args() -> QROptions {
    /*
        default options are....
            if no version, the default version is 21


        to do:
            flag for encoding type - default will be utf-8 (i think?)
            ???

    */
    let mut qr_args = args_os();
    let mut version = 8usize;
    let mut encoding = 8u8;
    let mut arg = qr_args.next();

    while arg.is_some() {
        let value = arg.unwrap();
        if value == OsStr::new("-v") {
            version = match qr_args.next() {
                Some(n) => {
                    let x = n.to_str().unwrap().parse::<usize>();
                    match x {
                        Ok(nx) if nx < 81 => nx,
                        Ok(_) => 21usize,
                        Err(_) => 21usize
                    }
                },
                None => 21usize
            }
        }


        arg = qr_args.next();
    }

    println!("version: {}, encoding: {}, square_count: {}", version, encoding, square_count(version));
    QROptions {
        version: version,
        encoding: encoding,
        requires_alignment: version > 1,
        finder_points: [
            (0, 0),
            ((square_count(version) - 7) - 1, 0),
            (0, (square_count(version) - 7) - 1)
        ]
    }
}

pub enum CellType {
    Finder,
    Alignment,
    Separator,
    Timing,
    DarkModule,
    Format,
    Free,
    Message,
    None
}

pub struct Color {
    r: u32,
    g: u32,
    b: u32
}
pub struct Cell {
    pub module_type: CellType,
    pub value: u8,
    pub point: Point,
    pub color: Color
}

pub struct Point(usize, usize);

struct QR {
    config: QROptions,
    body: Vec<Cell>
}

impl QR {
    fn setup(&mut self) {
        self.config.apply_finder_patterns(&mut self.body);
    }
}

impl QROptions {
    pub fn create_body(&self) -> Vec<Cell> {
        let mut rows: Vec<Cell> = vec![];
        let row_len = square_count(self.version) - 1;
        for x in 0..row_len {
            for y in 0..row_len {
                let cell = Cell {
                    point: Point(x as usize, y as usize),
                    value: 0,
                    color: Color { r: 0, g: 0, b: 0 },
                    module_type: CellType::None
                };
                rows.push(cell);
            }
        }
        rows
    }

    pub fn apply_finder_patterns(&self, body: &mut Vec<Cell>) {
        let row_length = square_count(self.version) - 1;

        for alignment_point in self.finder_points.iter() {
            let (mut x, mut y) = *alignment_point;
            let mut max = 6;
            while max > 1 {
                println!("I probably went too far! x: {}, y: {}, row length: {}, max: {}, size: {}", x, y, row_length, max, body.len());
                let mut cell_steps = max * 4;
                while cell_steps > 0 {
                    let idx = (row_length * x) + y;
                    let cell = body.get_mut(idx);
                    println!("X: {}, Y: {}", x, y);
                    match cell {
                        Some(c) => {
                            c.module_type = CellType::Finder;
                            if max == 4 {
                                c.color = Color { r: 255, g: 255, b: 255 }
                            }
                        },
                        None => panic!("I probably went too far! x: {}, y: {}, row length: {}, max: {}, idx: {}, steps: {}", x * row_length, y, row_length, max, idx, cell_steps)
                    }

                    if cell_steps > 3 * max {
                        y += 1;
                    } else if cell_steps > 2 * max {
                        x += 1;
                    } else if cell_steps > max {
                        y -= 1;
                    } else {
                        x -= 1;
                    }

                    cell_steps -= 1;

                }
                max -= 2;
                x += 1;
                y += 1;
            }
            let cell = body.get_mut((x * row_length) + y);
            match cell {
                Some(c) => c.module_type = CellType::Finder,
                None => {}
            }
        }

        // don't forget to handle center point here
    }
}

fn create_qr_image(qr: QR) {
    let mut img = ImageBuffer::new(49 * 20, 49 * 20);
    for cell in qr.body {
        for pixel in get_pixel_points(&cell) {
            let (x, y, color) = pixel;
            let rgb = Rgba { data: [color.r as u8, color.g as u8, color.b as u8, 100] };
            img.put_pixel(x, y, rgb);
        }
    }

    let ref mut fout = File::create(&Path::new("qr.png")).unwrap();
    // img.save();
    let _ = image_lib::ImageRgba8(img).save(fout, image_lib::PNG);

    // let _ = image_lib::ImageRgb8(img).save(fout, image_lib::PNG);
}


fn main() {
    let qr_version = 1;
    let size = 49;
    let message = String::from("www.wikipedia.org - here you can find junk and stuff and whatever and some things of greater importance i just want a longer byte length please thanks");
    let opts: QROptions = args();
    let mut qr: QR = QR {
        body: opts.create_body(),
        config: opts
    };
    qr.setup();
    create_qr_image(qr);


    // args();
    // create_grid(size, 2, qr_version, message);
}
