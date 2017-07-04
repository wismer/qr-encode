
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
    let mut version = 14usize;
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
        size: (((version - 1) * 4) + 21),
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

#[derive(Copy, Clone, Debug)]
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

#[derive(Debug)]
pub struct Point(usize, usize);

#[derive(Debug)]
pub struct PlotPoint {
    pub point: Point,
    pub color: Color
}

impl Point {
    pub fn idx(&self, size: usize) -> usize {
        (self.0 * size) + self.1
    }
}

struct QR {
    config: QROptions,
    body: Vec<Cell>
}

impl QR {
    fn setup(&mut self) {
        for alignment_point in self.config.finder_points.iter() {
            self.config.apply_finder_patterns(&mut self.body, *alignment_point);
            self.config.apply_separators(&mut self.body, *alignment_point);
        }

        if self.config.version != 1 {
            let alignment_points = self.config.get_alignment_points(&self.body);
            self.config.apply_alignment_patterns(&mut self.body, &alignment_points);
        }

        self.config.apply_timer_patterns(&mut self.body);
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
                    color: Color { r: 255, g: 255, b: 255 },
                    module_type: CellType::None
                };
                rows.push(cell);
            }
        }
        rows
    }

    pub fn apply_alignment_patterns(&self, body: &mut Vec<Cell>, points: &Vec<PlotPoint>) {
        for plot_point in points {
            let idx = plot_point.point.idx(self.size - 1);
            match body.get_mut(idx) {
                Some(cell) => {
                    cell.module_type = CellType::Alignment;
                    cell.color = plot_point.color
                },
                None => {}
            }
        }
    }

    pub fn get_alignment_points(&self, body: &Vec<Cell>) -> Vec<PlotPoint> {
        let mut pts: Vec<usize> = vec![];
        let mut n = 6;
        // let last_column = self.size - 7;
        let version_bracket = match self.version {
            1 => 0,
            2...7 => 1,
            7...13 => 2,
            14...21 => 3,
            22...28 => 4,
            29...36 => 5,
            37...41 => 6,
            _ => 0
        };

        let modifier = (self.size - 13) / version_bracket;
        while n <= self.size - 7 {
            pts.push(n);
            if n + modifier >= self.size - 7 {
                n += modifier - 1;
            } else {
                n += modifier;
            }
        }


        let pts: Vec<PlotPoint> = self.get_point_combinations(pts)
            .into_iter()
            .filter(|pt| {
                let idx = pt.idx(self.size - 1);
                let cell_ref = body.get(idx);
                if cell_ref.is_none() {
                    return false
                }

                let cell = cell_ref.unwrap();
                let result = match cell.module_type {
                    CellType::None => true,
                    _ => false
                };

                println!("{:?}, {}", pt, result);

                result
            })
            .flat_map(|pt| {
                self.plot_spiral(&pt, 4)
            })
            .collect();

        pts
    }

    pub fn get_point_combinations(&self, numbers: Vec<usize>) -> Vec<Point> {
        let mut pairs: Vec<Point> = vec![]; //numbers.iter().map(|n| (*n, *n)).collect();
        let xnumbers: Vec<usize> = numbers.iter().cloned().collect();
        for n in numbers {
            for xn in xnumbers.iter() {
                pairs.push(Point(n, *xn));
            }
        }
        pairs
    }

    pub fn apply_separators(&self, body: &mut Vec<Cell>, alignment_point: (usize, usize)) {
        let row_len = self.size - 1;
        let (mut x, mut y) = alignment_point;
        // x == y Upper left
        // x < y Upper Right
        // x > y Lower Left
        let mut start_x = 0;
        let mut start_y = 0;
        let mut end_x = 0;
        let mut end_y = 0;
        if x == y {
            // upper left
            start_x = 7;
            end_y = 7;
        } else if x > y {
            // lower left
            start_x = row_len - 8;
            end_x = row_len;
            end_y = 7;
        } else {
            // upper right
            start_y = row_len - 8;
            end_y = row_len;
            end_x = 7;
        }
        x = start_x;
        y = start_y;
        loop {
            let pt = Point(x, y);
            let idx = pt.idx(self.size - 1);
            match body.get_mut(idx) {
                Some(c) => {
                    c.module_type = CellType::Separator;
                    c.color = Color { r: 20, g: 255, b: 255 };
                },
                None => panic!("dunno idx {} x: {} y: {}", idx, x, y)
            }

            if start_x == end_y && y < end_y {
                y += 1;
            } else if end_y == y && x > end_x {
                x -= 1;
            } else if end_x > x && start_y > x {
                x += 1;
            } else if end_x == x && end_y - 1 > y {
                y += 1;
            } else if end_y > y && start_x > y {
                y += 1;
            } else if (end_x > 0 && end_x - 1 > x) && end_y == y {
                x += 1;
            } else {
                break;
            }
        }
    }

    pub fn plot_spiral(&self, origin_pt: &Point, size: usize) -> Vec<PlotPoint> {
        let mut plot_points: Vec<PlotPoint> = vec![];
        let mut max = size;
        let mut depth = 0;
        let (mut x, mut y) = (origin_pt.0 - 2, origin_pt.1 - 2);
        while max > 1 {
            let mut cell_steps = max * 4;
            let color = match depth % 2 {
                0 => Color { r: 0, g: 0, b: 0 },
                _ => Color { r: 255, g: 255, b: 255 },
            };
            while cell_steps > 0 {
                let plot_point = PlotPoint { point: Point(x, y), color: color };
                // println!("{:?}", color);
                plot_points.push(plot_point);
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
            depth += 1;
            max -= 2;
            x += 1;
            y += 1;
        }
        plot_points.push(PlotPoint { point: Point(x, y), color: Color { r: 30, g: 86, b: 240 } });
        plot_points
    }

    pub fn apply_finder_patterns(&self, body: &mut Vec<Cell>, alignment_point: (usize, usize)) {
        let row_length = square_count(self.version) - 1;
        let (mut x, mut y) = alignment_point;
        let mut max = 6;
        while max > 1 {
            let mut cell_steps = max * 4;
            while cell_steps > 0 {
                let point = Point(x, y);
                let idx = point.idx(self.size - 1);
                let cell = body.get_mut(idx);
                match cell {
                    Some(c) => {
                        c.module_type = CellType::Finder;
                        // change to white
                        if max != 4 {
                            c.color = Color { r: 0, g: 0, b: 0 }
                        }
                    },
                    None => panic!("I probably went too far! x: {}, y: {}, row length: {}, max: {}, idx: {}, size: {}", x * row_length, y, row_length, max, idx, self.size)
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
            Some(c) => {
                c.module_type = CellType::Finder;
                c.color = Color { r: 0, g: 0, b: 0 };
            },
            None => {}
        }
    }

    pub fn apply_timer_patterns(&self, body: &mut Vec<Cell>) {
        let (mut x, mut y) = (6, self.size - 8);
        loop {
            if x >= self.size - 7 {
                break;
            }

            let idx = (x * self.size - 1) + y;
            match body.get_mut(idx) {
                Some(cell) => {
                    match cell.module_type {
                        CellType::None => {
                            cell.module_type = CellType::Timing;
                            if idx % 2 == 0 {
                                cell.color = Color { r: 0, g: 0, b: 0 };
                            }
                        },
                        _ => {}
                    }
                },
                None => {}
            }
            if y > x {
                y -= 1;
            } else if y == 7 {
                y = 6;
                x = 8;
            } else {
                x += 1;
            }
        }
    }
}

fn create_qr_image(qr: QR) {
    let mut img = ImageBuffer::new((qr.config.size as u32)  * 20, (qr.config.size as u32) * 20);
    for cell in qr.body {
        for pixel in get_pixel_points(&cell) {
            let (x, y, color) = pixel;
            if x % 20 == 0 || y % 20 == 0 {
                let rgb = Rgba { data: [125, 125, 125, 100] };
                img.put_pixel(x, y, rgb);

            } else {
                let rgb = Rgba { data: [color.r as u8, color.g as u8, color.b as u8, 100] };
                img.put_pixel(x, y, rgb);

            }
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
