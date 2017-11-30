use qr_encoder::cell::{
    Cell,
    Point,
    CellType,
    Color
};

use qr_encoder::position::Position;
use qr_encoder::config::{QRConfig};

pub struct QR {
    pub body: Vec<Cell>,
    pub current_position: Position,
    pub previous_position: Position
}


impl QR {
    pub fn encode_meta(&mut self, config: &QRConfig) {
        let data_length = config.data.len() as u8;
        let size_message = config.get_content_length();
        let mode = config.encoding;

        self.encode_chunk(&mode, 4, config); // the encoding mode block
        self.encode_chunk(&data_length, 8, config);

        if size_message > 8 {
            let remaining_bits = data_length.rotate_right(size_message as u32);
            self.encode_chunk(&remaining_bits, size_message - 8, config);
        }
    }

    pub fn encode_chunk(&mut self, chunk: &u8, chunk_length: usize, config: &QRConfig) {
        let row_length = config.size - 1;
        let corners: [(isize, isize); 4] = [
            (-1, 1),
            (1, 1),
            (1, -1),
            (-1, -1)
        ];
    
        for i in 0..chunk_length {
            let mut position = Position {
                free: 0,
                algn: 0,
                timing: 0,
                off: 0,
                msg: 0,
                current_index: self.current_position.current_index,
                prev_index: self.current_position.prev_index
            };

            let current_point: Point = Point::as_point(position.current_index, config.size);

            let bit = chunk & (1 << (chunk_length - i) - 1);
            // let color = set_color(i);
            let color: Color = if bit == 0 {
                Color { r: 255, g: 255, b: 255 }
            } else {
                Color { r: 0, g: 0, b: 0 }
            };

            let mut corner_idx = 0;

            match self.body.get_mut(position.current_index) {
                Some(cell) => {
                    // println!("{:?}", cell);
                    cell.module_type = CellType::Message;
                    cell.color = color;
                },
                None => {
                    println!("this should never happen {:?}", current_point);
                }
            }

            while corner_idx < 4 {
                let corner = corners[corner_idx];
                let next_point = current_point >> corner;
                let next_idx = next_point.idx(config.size);
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

                if off_edge || next_point.1 >= config.size {
                    position.off ^= 1 << corner_idx;
                    corner_idx += 1;
                    continue;
                }
                let cell = cell_ref.unwrap();

                match cell.module_type {
                    CellType::None => {
                        // set no bits
                        position.free ^= 1 << corner_idx;
                    },

                    CellType::Timing => {
                        // set bits but not yet
                        position.timing ^= 1 << corner_idx;
                    },

                    CellType::Message => {
                        position.msg ^= 1 << corner_idx;
                    },

                    CellType::Alignment => {
                        position.algn ^= 1 << corner_idx;
                    },

                    _ => {
                        // set bits but not yet
                        position.off ^= 1 << corner_idx;
                    }
                }
                corner_idx += 1;
            }

            // after each corner gets examined, copy the current position context and save it to the previous position context
            self.previous_position = self.current_position;
            self.current_position = position.adjust_position(config.size, self.previous_position);
        }
    }

    pub fn setup(&mut self, config: &QRConfig) {
        for alignment_point in config.finder_points.iter() {
            let point = Point(alignment_point.0, alignment_point.1);
            config.apply_finder_patterns(&mut self.body, point);
            config.apply_separators(&mut self.body, *alignment_point);
        }

        if config.version != 1 {
            let alignment_points = config.get_alignment_points(&self.body);
            config.apply_alignment_patterns(&mut self.body, &alignment_points);
        }

        config.apply_timer_patterns(&mut self.body);
        config.apply_dark_module(&mut self.body);
        config.apply_reserve_format_areas(&mut self.body);

        // version information area
        if config.version > 6 {
            config.apply_version_information_areas(&mut self.body);
        }

        println!("LENGTH IS {}, SIZE IS {}, VERSION: {}", self.body.len(), config.size, config.version);
        println!("--- QR ENCODER READY FOR ENCODING ---");
    }
}
