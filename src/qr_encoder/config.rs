use qr_encoder::cell::{
    Cell,
    Point,
    CellType,
    Color,
    PlotPoint
};
use qr_encoder::util::{CodeWord, codeword_info};

pub enum ECLevel {
    Low,
    Medium,
    Q,
    High,
}

pub enum EncodingMode {
    Numeric,
    AlphaNumeric,
    Byte,
    Japanese
}

pub struct QRConfig {
    pub version: usize,
    pub data: Vec<u8>,
    pub codewords: Vec<u8>,
    pub codeword_properties: CodeWord,
    pub encoding: u8, // for now - should be its own sub-type.
    pub encoding_mode: EncodingMode,
    pub requires_alignment: bool,
    pub finder_points: [(usize, usize); 3],
    pub size: usize,
    pub err_correction_level: ECLevel
}

impl QRConfig {
    pub fn verify_version(&mut self) {
        // TODO!
        // let content_length = self.get_content_length();
        // println!("{:?} CL: {}", self.codeword_properties, content_length);
        // let data_length = self.data.len();

        // if data_length + 2 > self.codeword_properties.ecc_codeword_count {
        //     // data content is too large for the version size, so change the version to match.
        //     // TODO
        // } else if data_length + 2 < self.codeword_properties.ecc_codeword_count {
        //     // TODO: if someone wants to use version 20 when the data is only 4 bytes...

        // }
    }

    pub fn debug_data(&self) {
        let ref data = self.data;
        let ref codewords = self.codewords;
        for (idx, byte) in codewords.iter().enumerate() {
            if let Some(data_byte) = data.get(idx + 2) {
                println!("{:b} original", 112 << 4);
            }
            println!("{:b} byte #: {} ", idx, byte);
        }
    }

    pub fn translate_data(&mut self) {
        // let mut data_length = self.data.len();
        let mut data_length = 237usize;
        let mut content_length = self.get_content_length();
        let mut encoding = self.encoding;
        let copied_data = self.data.clone();

        {
            let mut codewords = &mut self.codewords;
            let mut current_byte = (encoding << 4) ^ data_length.rotate_right(4) as u8;

            codewords.push(current_byte);
            codewords.push(data_length.rotate_left(4) as u8);

            for byte in copied_data.iter() {
                {
                    let prev = codewords.last_mut().unwrap();
                    *prev ^= byte.wrapping_shr(4);
                }

                codewords.push(byte.wrapping_shl(4));
            }
        }
    }

    pub fn create_body(&self) -> Vec<Cell> {
        // this can be refactored so it just iterates going from 0 to max-index
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

    pub fn get_content_length(&self) -> usize {
        let modifier = match self.version {
            1...10 => 0,
            11...27 => 2,
            _ => 4
        };
        match self.encoding {
            1 => 10 + modifier,
            2 => 9 + modifier,
            8 => 12 + modifier,
            _ => {
                if self.version < 10 {
                    8
                } else {
                    16
                }
            }
        }
    }

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

    pub fn apply_reserve_format_areas(&self, body: &mut Vec<Cell>) {
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
                cell.color = Color { r: 10, g: 140, b: 230 };
            },
            None => {}
        }
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

                // println!("{:?}, {}", pt, result);

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
        plot_points.push(PlotPoint { point: Point(x, y), color: Color { r: 0, g: 0, b: 0 } });
        plot_points
    }
}
