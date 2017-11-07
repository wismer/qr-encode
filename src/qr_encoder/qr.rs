use qr_encoder::cell::{
    Cell,
    Point,
    CellType,
    Color,
    PlotPoint
};
use qr_encoder::util::{set_color};
use qr_encoder::area::Area;


pub struct QR {
    pub config: QROptions,
    pub body: Vec<Cell>
}

pub struct BitMap {
    pub map: usize
}

pub struct QROptions {
    pub version: usize,
    pub encoding: u8, // for now - should be its own sub-type.
    pub requires_alignment: bool,
    pub finder_points: [(usize, usize); 3],
    pub size: usize
}

pub enum EncoderEvent {
    ZigZag, // normal behavior
    OffEdge, // hit's an edge.
    FinderBlock, // hits the finder
    TracerBlock, // tracer
    AlignmentBlock, // alignment
    EndOfEncode // etc
}

fn bit_count(n: u8) -> u8 {
    let mut c = 0u8;
    let mut num = n;

    while num > 0 {
        num &= num - 1;
        c += 1;
    }

    c
}

fn lead_bit_position(n: u8) -> u8 {
    let mut i = 3u8;
    let mut pos = 1 << i;

    loop {
        if pos & n == pos || i == 0 {
            break;
        }

        i -= 1;
        pos = 1 << i;
    }

    i
}

impl QROptions {
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
                cell.color = Color { r: 255, b: 233, g: 20 }
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
        plot_points.push(PlotPoint { point: Point(x, y), color: Color { r: 30, g: 86, b: 240 } });
        plot_points
    }
}


impl QR {
    pub fn encode_chunk(&mut self, chunk: u8, position: (usize, usize)) -> (usize, usize) {
        let mut current_index = position.0;
        let mut prev_index = position.1;
        let row_length = self.config.size - 1;
        let corners: [(isize, isize); 4] = [
            (-1, 1),
            (1, 1),
            (1, -1),
            (-1, -1)
        ];

        for i in 0..8 {
            let current_point: Point = Point::as_point(current_index, self.config.size);
            // let bit = chunk & (1 << i);
            let color = set_color(i);
            let mut area = Area {
                free: 0,
                msg: 0,
                off: 0,
                algn: 0,
                finder: 0
            };
            let mut corner_idx = 0;

            match self.body.get_mut(current_index) {
                Some(cell) => {
                    // println!("{:?}", cell);
                    cell.module_type = CellType::Message;

                    if i == 7 {
                        cell.color = Color { r: 0, g: 0, b: 0 };
                    } else {
                        cell.color = color;
                    }
                },
                None => {
                    println!("this should never happen {:?}", current_point);
                }
            }

            while corner_idx < 4 {
                let corner = corners[corner_idx];
                let next_point = current_point >> corner;
                let next_idx = next_point.idx(self.config.size);
                let cell_ref = self.body.get(next_idx);
                let off_edge = cell_ref.is_none();

                /*
                CMOA

                C = Corner (positions 15, 14, 13, 12)
                M = Message (positions 11, 10, 9, 8)
                O = Off Limit (positions 7, 6, 5, 4)
                A = Alignment (positions 3, 2, 1, 0)
                first cell would look like...

                    C 0001 M 0000 O 1110 A 0000

                second...

                    C 1001 M 0000 O 0110 A 0000

                third

                    C 0001 M 0010 O 1100 A 0000



                */

                if off_edge || next_point.1 >= self.config.size {
                    area.off ^= 1 << corner_idx;
                    corner_idx += 1;
                    continue;
                }
                let cell = cell_ref.unwrap();

                match cell.module_type {
                    CellType::None => {
                        // set no bits
                        area.free ^= 1 << corner_idx;
                    },

                    CellType::Finder => {
                        // set bits but not yet
                        area.finder ^= 1 << corner_idx;
                    },

                    CellType::Message => {
                        area.msg ^= 1 << corner_idx;
                    },

                    CellType::Alignment => {
                        area.algn ^= 1 << corner_idx;
                    },

                    _ => {
                        // set bits but not yet
                        area.off ^= 1 << corner_idx;
                    }
                }
                corner_idx += 1;
            }

            let lead = lead_bit_position(area.free);
            let set_bits = bit_count(area.free);
            let former_position = prev_index;
            prev_index = current_index;

            // handle offsides cells
            if area.off > 0 {
                if area.off == 0b1001 && area.free == 0 {
                    current_index -= 1;
                } else if area.off == 0b1001 && set_bits == 1 {
                    current_index -= 1;
                } else if area.off == 0b0001 && set_bits >= 2 {
                    current_index += row_length + 2;
                } else if area.free ^ area.off == 0b1111 && lead == 0b11 && set_bits == 1 {
                    current_index -= 1;
                } else if area.free ^ area.off == 15 && area.free == 0b1001 {
                    current_index -= row_length;
                } else if area.free ^ area.off == 15 && area.free == 0b0110 {
                    current_index += row_length + 2;
                } else if area.free == 0 && area.msg ^ area.off == 0b1111 {
                    current_index -= 1;
                } else if area.msg == 0b0100 || area.msg == 0b1000 || area.msg == 0b0010 {
                    current_index -= 1;
                } else if area.free == 0b0110 && area.off == 0b1001 {
                    current_index += row_length;
                } else if area.free == 0b1000 {
                    current_index -= 1;
                }
            } else if area.algn > 0 {
                current_index = self.by_alignment(area, former_position, current_index);
            } else {
                // free of edge concerns
                if area.msg == 0b0010 {
                    current_index -= row_length;
                } else if area.msg == 0b0001 {
                    current_index += row_length + 2;
                } else {
                    current_index -= 1;
                }
            }

            /*
            trying to get algn = 1001 (top)
                                 0110 (bottom)
                                 1000 (upper left corner)
                                 0011 (left)



            along edge would be like

            0b11110011
            0b11111100
            0b00111111
            0b11001111

            alternatively...

            corner = 0b0
            along x axis = 0b100
            along y axis = 0b001
            x edge = 0b10
            y edge = 0b01
            corner = 0b11

            */
        }

        (current_index, prev_index)
    }

    fn by_alignment(&self, area: Area, prev_index: usize, index: usize) -> usize {
        let size = self.config.size;
        let msg = area.msg;
        let free = area.free;
        let algn = area.algn;

        let free_count = bit_count(free);
        let msg_count = bit_count(msg);

        if msg == 0b1001 || msg == 0b0110 {
            return index - 1
        }

        if 0b1001 & free == 0b1001 {
            return index - size + 1
        }

        if free == 0b1001 || free == 0b0101 {
            index - size + 1
        } else if free == 0b0110 || free == 0b1010 {
            index + size + 1
        } else if algn == 0b1100 && free > 0 {
            index + size + 1
        } else if algn == 0b1001 && index + 1 == prev_index {
            index - (size * 6) + 1
        } else if algn == 0b0110 {
            // panic!("{:?}", Point::as_point(index + (size * 6) + 1, size));
            index + (size * 6) + 1
        } else if algn == 1 {
            index - size
        } else if algn == 0b0011 && prev_index + 1 != index {
            index - size
        } else {
            index - 1
        }

        // if algn == 0b0100 && free ^ msg == 0b1011 {
        //     match free {
        //         0b0010 => {},

        //     }
        // }

        // if algn == 0b1001 && msg > 0 {
        //     index - (size * 6)
        // } else if algn == 0b0110 && msg > 0 {
        //     index + (size * 6)
        // } else if algn == 0b0010 && msg > 0 {
        //     index + size + 1
        // } else if algn == 0b0100 && msg == 1 {
        //     index + size + 1
        // } else if algn == 0b1100 && msg == 1 {
        //     index + size + 1
        // } else if algn == 0b1100 && prev_index < index {
        //     index + size + 1
        // } else if algn == 0b1000 && msg > 0 {
        //     index + size + 1
        // } else if algn == 0b1000 && msg == 0b0111 {
        //     index - 1
        // } else {
        //     index
        // }
    }

    pub fn setup(&mut self) {
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
        self.config.apply_reserve_format_areas(&mut self.body);

        // version information area
        if self.config.version > 6 {
            self.config.apply_version_information_areas(&mut self.body);
        }

        println!("LENGTH IS {}, SIZE IS {}", self.body.len(), self.config.size);
        println!("--- QR ENCODER READY FOR ENCODING ---");
    }
}
