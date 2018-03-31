use qr_encoder::cell::{
    Cell,
    Point,
    CellType,
    Color
};

use qr_encoder::config::{QRConfig};
use qr_encoder::util::{set_color, get_index_value};
use qr_encoder::cursor::Cursor;

pub struct QR {
    pub body: Vec<Cell>,
    pub cursor: Cursor
}

impl QR {
    pub fn encode_chunk(&mut self, chunk: &u8, chunk_length: usize, config: &QRConfig) {
        let cursor = &mut self.cursor;
        let corners: [(isize, isize); 8] = [
            (-1, 0), // UP
            (-1, 1), // UR
            (0, 1), // RIGHT
            (1, 1), // LR
            (1, 0), // BOTTOM
            (1, -1), // LL
            (0, -1), // LEFT
            (-1, -1), // UL
        ];

        for i in 0..chunk_length {
            let current_index = cursor.current_index;
            let bit = chunk & (1 << (chunk_length - i) - 1);

            let mut corner_idx = 0;

            match self.body.get_mut(current_index) {
                Some(cell) => {
                    match cell.module_type {
                        CellType::None => {
                            let color: Color = if config.debug_mode {
                                set_color(i)
                            } else if bit == 0 {
                                cell.value = bit;
                                Color { r: 255, g: 255, b: 255 }
                            } else {
                                cell.value = 1;
                                Color { r: 0, g: 0, b: 0 }
                            };

                            cell.module_type = CellType::Message;
                            cell.color = color;
                        },
                        _ => {
                            if config.debug_mode {
                                panic!("NO");
                            }
                        }
                    }
                },
                None => {
                    panic!("this should never happen {:?}", current_index);
                }
            }

            while corner_idx < 8 {
                // unnecessary conversion occuring here. FIXME
                let corner = corners[corner_idx];
                let cell_ref = match get_index_value(current_index as isize, corner, config.size as isize) {
                    Some(next_idx) => self.body.get(next_idx),
                    None => None
                };

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

                if off_edge {
                    cursor.context.off ^= 1 << corner_idx;
                    corner_idx += 1;
                    continue;
                }

                let cell = cell_ref.unwrap();

                match cell.module_type {
                    CellType::None => {
                        // set no bits
                        cursor.context.free ^= 1 << corner_idx;
                    },

                    CellType::Timing => {
                        // set bits but not yet
                        cursor.context.timing ^= 1 << corner_idx;
                    },

                    CellType::Message => {
                        cursor.context.msg ^= 1 << corner_idx;
                    },

                    CellType::Alignment => {
                        cursor.context.algn ^= 1 << corner_idx;
                    },

                    _ => {
                        // set bits but not yet
                        cursor.context.off ^= 1 << corner_idx;
                    }
                }
                corner_idx += 1;
            }

            cursor.move_cursor(config.size);

            // after each corner gets examined, copy the current position context and save it to the previous position context
            // overwrite the previous position to the current one (which is really just one step behind)
            // three points:
            // 1: where the cursor was BEFORE the state changes
            // let previous_position_index = self.previous_position.current_index;
            // self.previous_position = self.current_position;
            // self.previous_position.prev_index = previous_position_index;
            // // then set the current position with the proper context
            // self.current_position = position.adjust_position(config.size, self.previous_position);
            // if self.current_position.timing > 0 {
            //     println!("{}", (self.current_position.current_index as isize) - (self.previous_position.prev_index as isize));
            //     println!("curr {}, prev {}, prev prev: {}", self.current_position.current_index, self.current_position.prev_index, self.previous_position.prev_index);
            // }
        }
    }

    pub fn setup(&mut self, config: &mut QRConfig) {
        config.translate_data(); // interleaves data if necessary
        config.encode_error_correction_codewords();

        for alignment_point in config.finder_points.iter() {
            let point = Point(alignment_point.0, alignment_point.1);
            config.apply_finder_patterns(&mut self.body, point);
            config.apply_separators(&mut self.body, *alignment_point);
        }

        if config.version != 1 {
            let alignment_points = config.get_alignment_points(&self.body);
            config.apply_alignment_patterns(&mut self.body, &alignment_points);
        }


        config.apply_reserve_format_areas(&mut self.body);
        config.apply_dark_module(&mut self.body);
        config.apply_timer_patterns(&mut self.body);

        // version information area
        if config.version > 6 {
            config.apply_version_information_areas(&mut self.body);
        }

        println!("LENGTH IS {}, SIZE IS {}, VERSION: {}", self.body.len(), config.size, config.version);
        println!("--- QR ENCODER READY FOR ENCODING ---");
    }

    pub fn encode_data(&mut self, config: &QRConfig) {
        {
            let data = &config.codewords[..];
            for byte in data.iter() {
                self.encode_chunk(&byte, 8, config);
            }

            let remainder_bits = &config.get_remainder_bit_length();
            self.encode_chunk(&0, *remainder_bits, config);
        }

        {
            let best_pattern = self.get_best_mask_pattern(&config);
            config.apply_mask_pattern(&mut self.body, best_pattern);
            config.encode_format_areas(&mut self.body, best_pattern as u8);
        }
    }

    fn get_best_mask_pattern(&self, config: &QRConfig) -> usize {
        let body = &self.body;
        let mut best = 0;
        let mut best_pattern = 0;
        for pattern in 0..7 {
            let mut copy = &mut body.clone();
            config.apply_mask_pattern(&mut copy, pattern);
            let score = config.eval_penalty_scores(copy);
            if best == 0 || score < best {
                best = score;
                best_pattern = pattern;
            }
        }

        best_pattern
    }
}
