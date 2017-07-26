
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

pub enum CellType {
    Finder,
    Alignment,
    Separator,
    Timing,
    DarkModule,
    VersionInformation,
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

pub fn get_pixel_points(cell: &Cell) -> Vec<(u32, u32, Color)> {
    let i = (cell.point.0 * 20) as u32;
    let j = (cell.point.1 * 20) as u32;
    let mut pixels: Vec<(u32, u32, Color)> = vec![];
    for row in i..(i + 20) {
        for col in j..(j + 20) {
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
    let encoding = 8u8;
    let mut arg = qr_args.next();

    while arg.is_some() {
        let value = arg.unwrap();
        if value == OsStr::new("-v") {
            version = match qr_args.next() {
                Some(n) => {
                    let x = n.to_str().unwrap().parse::<usize>();
                    match x {
                        Ok(nx) if nx < 81 => nx, // if it fails to parse, or parses a number greater than 81, set it to version 21.
                        Ok(_) => 21usize,
                        Err(_) => 21usize
                    }
                },
                None => 21usize
            }
        }


        arg = qr_args.next();
    }

    QROptions {
        version: version,
        encoding: 8u8,
        requires_alignment: version > 1,
        size: (((version - 1) * 4) + 21),
        finder_points: [
            (0, 0),
            ((square_count(version) - 7), 0),
            (0, (square_count(version) - 7))
        ]
    }
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

impl  QR {
    fn setup(&mut self) {
        for alignment_point in self.config.finder_points.iter() {
            let point = Point(alignment_point.0, alignment_point.1);
            self.config.apply_finder_patterns(&mut self.body, point);
            self.config.apply_separators(&mut self.body, *alignment_point);
        }

        if self.config.version != 1 {
            let alignment_points = self.config.get_alignment_points(&self.body);
            self.config.apply_alignment_patterns(&mut self.body, &alignment_points);
        }

        self.config.apply_timer_patterns(&mut self.body);
        self.config.apply_dark_module(&mut self.body);
        self.config.reserve_format_areas(&mut self.body);

        // version information area
        if self.config.version > 6 {
            self.config.apply_version_information_areas(&mut self.body);
        }

        println!("LENGTH IS {}, SIZE IS {}", self.body.len(), self.config.size);
    }
}

impl QROptions {
    pub fn apply_version_information_areas(&self, body: &mut Vec<Cell>) {
        let mut x = self.size - 11;
        let mut y = 0;
        let mut blocks = 6 * 3;
        while blocks > 0 {
            let indices: [usize; 2] = [
                Point(x, y).idx(self.size),
                Point(y, x).idx(self.size)
            ];
            for index in indices.into_iter() {
                match body.get_mut(*index) {
                    Some(cell) => {
                        cell.module_type = CellType::VersionInformation;
                        cell.color = Color { r: 200, g: 200, b: 123 };
                    },
                    None => {}
                }

            }

            if y < 5 {
                y += 1;
            } else {
                y = 0;
                x += 1;
            }
            blocks -= 1;
        }
    }

    pub fn reserve_format_areas(&self, body: &mut Vec<Cell>) {
        let mut vertical = Point(0, 8);
        let mut horizontal = Point(8, 0);

        while horizontal.1 < self.size {
            let idx = horizontal.idx(self.size);
            match body.get_mut(idx) {
                Some(cell) => {
                    cell.module_type = CellType::Format;
                    cell.color = Color { r: 10, g: 140, b: 230 };
                },
                None => {}
            }

            if horizontal.1 > 7 && horizontal.1 < self.size - 8 {
                horizontal = Point(8, self.size - 8);
            } else {
                horizontal = Point(8, horizontal.1 + 1);
            }
        }

        while vertical.0 < self.size {
            let idx = vertical.idx(self.size);
            match body.get_mut(idx) {
                Some(cell) => {
                    cell.module_type = CellType::Format;
                    cell.color = Color { r: 10, g: 140, b: 230 };
                },
                None => {}
            }

            if vertical.0 > 7 && vertical.0 < self.size - 8 {
                vertical = Point(self.size - 8, 8);
            } else {
                vertical = Point(vertical.0 + 1, 8);
            }
        }

    }

    pub fn apply_dark_module(&self, body: &mut Vec<Cell>) {
        let dark_module_coord = Point((4 * self.version) + 9, 8);
        let idx = dark_module_coord.idx(self.size);
        match body.get_mut(idx) {
            Some(cell) => {
                cell.module_type = CellType::DarkModule;
                cell.color = Color { r: 255, b: 233, g: 20 }
            },
            None => {}
        }
    }

    pub fn create_body(&self) -> Vec<Cell> {
        let mut rows: Vec<Cell> = vec![];
        let row_len = self.size;
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
            let idx = plot_point.point.idx(self.size);
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

        let modifier = (self.size - 12) / version_bracket;
        while n <= self.size - 7 {
            pts.push(n);
            n += modifier;
        }


        let pts: Vec<PlotPoint> = self.get_point_combinations(pts)
            .into_iter()
            .filter(|pt| {
                let idx = pt.idx(self.size);
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
                self.plot_spiral(&pt, 4, 2)
            })
            .collect();

        pts
    }

    pub fn get_point_combinations(&self, numbers: Vec<usize>) -> Vec<Point> {
        let mut pairs: Vec<Point> = vec![]; //numbers.iter().map(|n| (*n, *n)).collect();
        let xnumbers: Vec<usize> = numbers.iter().cloned().collect();
        for n in numbers {
            for xn in xnumbers.iter() { // can I use the same vec inside its iteration?
                pairs.push(Point(n, *xn));
            }
        }
        pairs
    }

    pub fn apply_separators(&self, body: &mut Vec<Cell>, alignment_point: (usize, usize)) {
        let row_len = self.size;
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
            let idx = pt.idx(self.size);
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

    pub fn plot_spiral(&self, origin_pt: &Point, size: usize, diff: usize) -> Vec<PlotPoint> {
        let mut plot_points: Vec<PlotPoint> = vec![];
        let mut max = size;
        let mut depth = 0;
        let (mut x, mut y) = (origin_pt.0 - diff, origin_pt.1 - diff);
        while max > 1 {
            let mut cell_steps = max * 4;
            let color = match depth % 2 {
                0 => Color { r: 0, g: 0, b: 0 },
                _ => Color { r: 255, g: 255, b: 255 },
            };
            while cell_steps > 0 {
                let plot_point = PlotPoint { point: Point(x, y), color: color };
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
        // center cell
        plot_points.push(PlotPoint { point: Point(x, y), color: Color { r: 30, g: 86, b: 240 } });
        plot_points
    }

    pub fn apply_finder_patterns(&self, body: &mut Vec<Cell>, alignment_point: Point) {
        for plot_point in self.plot_spiral(&alignment_point, 6, 0) {
            let idx = plot_point.point.idx(self.size);
            match body.get_mut(idx) {
                Some(cell) => {
                    cell.module_type = CellType::Finder;
                    cell.color = plot_point.color
                },
                None => {}
            }
        }
    }

    pub fn apply_timer_patterns(&self, body: &mut Vec<Cell>) {
        let (mut x, mut y) = (6, self.size - 8);
        loop {
            if x >= self.size - 7 {
                break;
            }
            let pt = Point(x, y);
            let idx = pt.idx(self.size);
            match body.get_mut(idx) {
                Some(cell) => {
                    match cell.module_type {
                        CellType::None => {
                            let direction = if y > x {
                                y
                            } else {
                                x
                            };
                            cell.module_type = CellType::Timing;
                            if direction % 2 == 0 {
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
    let dimensions: u32 = (qr.config.size) as u32;
    let mut img = ImageBuffer::new(dimensions * 20, dimensions * 20);
    for cell in qr.body {
        for pixel in get_pixel_points(&cell) {
            let (x, y, color) = pixel;
            if x % 20 == 0 || y % 20 == 0 {
                // cell border
                let rgb = Rgba { data: [125, 125, 125, 100] };
                img.put_pixel(x, y, rgb);
            } else {
                let rgb = Rgba { data: [color.r as u8, color.g as u8, color.b as u8, 100] };
                img.put_pixel(x, y, rgb);
            }
        }
    }

    let ref mut fout = File::create(&Path::new("qr.png")).unwrap();
    let _ = image_lib::ImageRgba8(img).save(fout, image_lib::PNG);
}

impl QR {
    fn encode_bit(&mut self, idx: usize, bit: u8) -> bool {
        let cell_ref = self.body.get_mut(idx);

        if cell_ref.is_none() {
            return false
        }

        let mut cell = cell_ref.unwrap();
        match cell.module_type {
            CellType::None => {
                cell.module_type = CellType::Message;
                if bit == 0 {
                    cell.color = Color { r: 0, g: 0, b: 0 };
                } else {
                    cell.color = Color { r: 24, g: 210, b: 100 };
                }
                true
            },
            _ => false
        }
    }

    pub fn encode_chunk(&mut self, chunk: u8, position: usize) -> usize {
        let mut current_pt = position;
        let mut prev_position = position;
        for i in 0..8 {
            let bit = chunk & (1 << i);
            prev_position = current_pt;

            let did_encode = self.encode_bit(current_pt, bit);

            if !did_encode {
                continue;
            }

            if i % 2 == 0 {
                current_pt = current_pt - 1;
            } else {
                current_pt = current_pt - (self.config.size - 1);
            }
        }

        current_pt
    }
}


fn main() {
    let opts: QROptions = args();
    let mut qr: QR = QR {
        body: opts.create_body(),
        config: opts
    };
    qr.setup();

    let sample = "\'It Was the Best of times, it was the Blurst of times??\'".to_string();
    let mut position = (qr.config.size * qr.config.size) - 1;
    for s in sample.into_bytes().into_iter() {
        position = qr.encode_chunk(s, position);
    }

    create_qr_image(qr);
}
