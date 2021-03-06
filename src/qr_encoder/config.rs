extern crate reed_solomon;

use std::ops::{BitXorAssign};
use std::fmt::{Binary, Debug};
use self::reed_solomon::{Encoder, Buffer};

use qr_encoder::cell::{
    Cell,
    Point,
    CellType,
    Color,
    PlotPoint
};
use qr_encoder::qr::QR;
use qr_encoder::util::{CodeWord, get_indices_for_dimensions};


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
    pub mask: usize,
    pub encoding: u8, // for now - should be its own sub-type.
    pub encoding_mode: EncodingMode,
    pub debug_mode: bool,
    pub requires_alignment: bool,
    pub finder_points: [(usize, usize); 3],
    pub size: usize,
    pub err_correction_level: ECLevel
}

const ECC_FORMAT_MASK: u16 = 21522;
const GEN_POLY_VERSION: u32 = 7973;
const GEN_POLY_FORMAT: u16 = 1335;



fn assign_bit_from_codeword(index: usize, body: &mut Vec<Cell>, dark: bool) -> isize {
    let cell = body.get_mut(index).unwrap();
    match cell.module_type {
        CellType::None => {
            if dark {
                cell.value = 1;
                cell.color = Color { r: 0, b: 0, g: 0 };
            } else {
                cell.value = 0;
                cell.color = Color { r: 255, g: 255, b: 255 };
            };
            cell.module_type = CellType::Message;

            -1
        },
        _ => 0
    }
}

fn zig_zag_points(canvas_size: usize) -> Vec<usize> {
    let mut col = canvas_size - 1;
    let mut row = canvas_size - 1;
    let mut indices = vec![];
    let mut inc = -1;
    while col != 0 {
        if col == 6 {
            col -= 1;
            continue;
        }
        for c in 0..2 {
            let index = (row * canvas_size) + col - c;
            indices.push(index);
        }
        
        if row == 0 && inc == -1 {
            inc = 1;
            col -= 2;
        } else if row == canvas_size - 1 && inc == 1 {
            inc = -1;
            if col == 1 {
                col -= 1;
            } else {
                col -= 2;
            }
        } else {
            row = (row as isize + inc) as usize;            
        }
    }
    
    indices
}

pub fn ecc_format_u32(data: u32, gen_poly: u32) -> u32 {
    let (limit, mut format_str) = (19, data << 12);
    
    while format_str.leading_zeros() <= limit {
        let diff = gen_poly.leading_zeros() - format_str.leading_zeros();
        format_str ^= gen_poly << diff as usize;
    }

    (data << 12) | format_str
}

pub fn ecc_format_u16(data: u16, gen_poly: u16, gen_mask: u16) -> u16 {
    let (limit, mut format_str) = (5, data << 10);

    while format_str.leading_zeros() <= limit {
        let diff = gen_poly.leading_zeros() - format_str.leading_zeros();
        format_str ^= gen_poly << diff as usize;
    }

    ((data << 10) | format_str) ^ gen_mask
}

// NOTE FOR MATT FOR TOMORROW ABOUT ISSUE WITH VERSIONS 4, 5 and 6 NOT WORKING -> CHECK THE ERROR ENCODING PROCESS FOR GROUPS THE ISSUE MIGHT BE THERE!

fn interleave_blocks(blocks: &[Buffer], block_size: usize, ecc_block_size: usize) -> Vec<u8> {
    let mut data: Vec<u8> = vec![];
    for i in 0..block_size {
        for block in blocks {
            if let Some(cw) = block.data().get(i) {
                data.push(*cw);
            } else {
                println!("bleh: {}, block_size: {}", i, block_size);
            }
        }
    }

    for i in 0..ecc_block_size {
        for block in blocks {
            if let Some(cw) = block.ecc().get(i) {
                data.push(*cw);
            } else {
                panic!("ADLKJSDLFKJD ECC {}", i);
            }
        }
    }

    data
}

pub struct CellTrack(pub Vec<usize>, pub Vec<usize>, pub Vec<usize>);

impl QRConfig {    
    pub fn gen_qr_code(&mut self) -> (QR, CellTrack) {
        self.translate_data();
        self.encode_error_correction_codewords();

        let mut canvas: QR = QR {
            body: self.create_body()
        };
        
        canvas.setup(&self);
        
        let tracker = self.process_data(&mut canvas);
        self.post_process_data(&mut canvas);
        
        (canvas, tracker)
    }

    fn process_data(&self, canvas: &mut QR) -> CellTrack {
        let mut bit_index = 7;
        let mut codeword_index = 0usize;
        let mut bit_order = vec![];
        let mut cw_order = vec![];
        let mut point_order = vec![];
        let pathing = zig_zag_points(self.size);
        let pathing_iter = &mut pathing.iter();
        
        // codewords
        while codeword_index < self.codewords.len() {
            let cw = self.codewords[codeword_index];
            bit_order.push(bit_index as usize);
            cw_order.push(codeword_index as usize);
            let idx = pathing_iter.next().unwrap();
            point_order.push(*idx as usize);
            bit_index += assign_bit_from_codeword(*idx, &mut canvas.body, (cw >> bit_index) & 1 == 1);

            if bit_index == -1 {
                bit_index = 7;
                codeword_index += 1;
            }
        }

        let mut remainder_bits = self.get_remainder_bit_length();
        while remainder_bits > 0 {
            let i = pathing_iter.next().unwrap();
            remainder_bits += assign_bit_from_codeword(*i, &mut canvas.body, false);
        }

        CellTrack(bit_order, cw_order, point_order)
    }
    
    fn post_process_data(&self, canvas: &mut QR) {
        let body = &mut canvas.body;
        let mut best = 0;
        let mut best_pattern = 0;
        for pattern in 0..7 {
            let mut copy = &mut body.clone();
            self.apply_mask_pattern(&mut copy, pattern);
            let score = self.eval_penalty_scores(copy);
            if best == 0 || score < best {
                best = score;
                best_pattern = pattern;
            }
        }

        self.apply_mask_pattern(body, best_pattern);
        self.encode_format_areas(body, best_pattern as u8);
        
        if self.version >= 7 {
            self.apply_version_information(body);
        }
    }
    
    pub fn get_ecc_length(&self) -> usize {
        self.codeword_properties.ecc_codeword_count
    }

    pub fn get_remainder_bit_length(&self) -> isize {
        match self.version {
            2...6 => 7,
            14...20 | 28...34 => 3,
            _ => 0
        }
    }

    pub fn get_mask_pattern(&self, n: usize) -> Box<Fn(usize, usize) -> bool> {
        match n {
            0 => Box::new(move |row: usize, col: usize| (row + col) % 2 == 0),
            1 => Box::new(move |row: usize, _: usize| row % 2 == 0),
            2 => Box::new(move |_: usize, col: usize| col % 3 == 0),
            3 => Box::new(move |row: usize, col: usize| (row + col) % 3 == 0),
            4 => Box::new(move |row: usize, col: usize| ((row / 2) + (col / 3)) % 2 == 0),
            5 => Box::new(move |row: usize, col: usize| ((row * col) % 2) + ((row * col) % 3) == 0),
            6 => Box::new(move |row: usize, col: usize| (((row * col) % 2) + ((row * col) % 3) ) % 2 == 0),
            _ => Box::new(move |row: usize, col: usize| (((row + col) % 2) + ((row * col) % 3) ) % 2 == 0)
        }
    }

    pub fn apply_version_information(&self, body: &mut Vec<Cell>) {
        let canvas_size = self.size as isize;
        let origin = (canvas_size - 12) as isize;
        let bit_string = ecc_format_u32(self.version as u32, GEN_POLY_VERSION);
        let upper_right_indices = get_indices_for_dimensions(origin, 1, canvas_size - 3);
        let lower_left_indices = get_indices_for_dimensions(origin * canvas_size, canvas_size, (-canvas_size * 3) + 1);
        // panic!("VERSION BINARY {:018b}", bit_string);

        for (bit_pos, i) in upper_right_indices.iter().enumerate() {
            let is_bit = ((bit_string >> bit_pos) & 1) > 0;
            let color: Color = if is_bit {
                Color { r: 0, b: 0, g: 0 }
            } else {
                Color { r: 255, b: 255, g: 255 }
            };

            match body.get_mut(*i) {
                Some(c) => c.color = color,
                None => {}
            }
        }

        for (bit_index, idx) in lower_left_indices.iter().enumerate() {
            let is_bit = (bit_string & (1 << bit_index)) > 0;
            let color: Color = if is_bit {
                Color { r: 0, b: 0, g: 0 }
            } else {
                Color { r: 255, b: 255, g: 255 }
            };
            match body.get_mut(*idx) {
                Some(c) => c.color = color,
                None => {}
            }
        }
    }

    pub fn encode_format_areas(&self, body: &mut Vec<Cell>, pattern: u8) {
        let ec_level: u8 = match self.err_correction_level {
            ECLevel::Low => 1,
            ECLevel::Medium => 0,
            ECLevel::Q => 3,
            _ => 2
        };

        let data = (ec_level << 3) | pattern;
        let format_str = ecc_format_u16(data as u16, GEN_POLY_FORMAT, ECC_FORMAT_MASK);

        let mut bit_position = 14;

        let mut x = 8;
        let mut y = 0;

        while y != self.size {
            let bit = format_str & (1 << bit_position);
            let color: Color = if bit == 0 {
                Color { r: 255, g: 255, b: 255 }
            } else {
                Color { r: 0, g: 0, b: 0 }
            };

            let idx = (x * self.size) + y;
            let cell = body.get_mut(idx).unwrap();
            match cell.module_type {
                CellType::Format => cell.color = color,
                CellType::Timing => {
                    if x == 8 {
                        y += 1;
                    } else {
                        x -= 1;
                    }
                    continue;
                },
                CellType::DarkModule => {
                    x = 8;
                    y = self.size - 8;
                    continue;
                }
                _ => panic!("What is this? {:?}", cell)
            }

            if x == 8 && (y < 8 || y >= self.size - 8) {
                y += 1;
            } else if y == 8 && x > 0 {
                x -= 1;
            } else if x == 0 && y == 8 {
                x = self.size - 1;
            }

            if bit_position == 0 {
                bit_position = 14;
            } else {
                bit_position -= 1;
            }
        }
    }

    pub fn apply_mask_pattern(&self, body: &mut Vec<Cell>, n: usize) {
        let pattern = self.get_mask_pattern(n);

        for cell in body {
            match cell.module_type {
                CellType::Message => {
                    let flip_module = pattern(cell.point.0, cell.point.1);
                    if flip_module && cell.is_black() {
                        cell.color = Color { r: 255, g: 255, b: 255 };
                    } else if flip_module {
                        cell.color = Color { r: 0, g: 0, b: 0 };
                    }
                },
                _ => {}
            }
        }
    }

    pub fn eval_penalty_scores(&self, body: &Vec<Cell>) -> usize {
        let one = self.penalty_score_eval_one(body);
        let two = self.penalty_score_eval_two(body);
        let three = self.penalty_score_eval_three(body);
        let four = self.penalty_score_eval_four(body);
        let total = one + two + three + four;

        total
    }

    pub fn penalty_score_eval_two(&self, body: &Vec<Cell>) -> usize {
        let mut penalty_total = 0;
        let canvas_size = self.size;

        let adjacent_coords = [
            canvas_size,
            canvas_size + 1,
            1
        ];

        for x in 0..(canvas_size - 1) {
            for y in 0..(canvas_size - 1) {
                let idx = (x * canvas_size) + y;
                let is_black = body[idx].is_black();
                let square = adjacent_coords.into_iter()
                    .map(|&i| body[i + idx].is_black())
                    .all(|p| p == is_black);

                if square {
                    penalty_total += 3;
                }
            }
        }

        penalty_total
    }


    fn check_column(&self, body: &Vec<Cell>, column: isize) -> usize {
        let mut subtotal = 0;
        let pattern_mask: u16 = 0b00001011101;
        let reverse_mask: u16 = 0b10111010000;
        let remap_pattern: u16 = 0b11111111111;
        let mut current_pattern = 0;
        for row in 0..self.size {
            let idx = (row * self.size) + column as usize;
            let cell = &body[idx];

            if cell.is_black() {
                current_pattern = ((current_pattern << 1) ^ 1) & remap_pattern;
            } else {
                current_pattern = (current_pattern << 1) & remap_pattern;
            }

            if row >= 9 && current_pattern == pattern_mask || current_pattern == reverse_mask {
                subtotal += 40;
            }
        }

        subtotal
    }

    fn check_row(&self, body: &Vec<Cell>, row: isize) -> usize {
        let mut subtotal = 0;
        let pattern_mask: u16 = 0b00001011101;
        let reverse_mask: u16 = 0b10111010000;
        let remap_pattern: u16 = 0b11111111111;
        let mut current_pattern = 0;
        for column in 0..self.size {
            let idx = ((row as usize) * self.size) + column as usize;
            let cell = &body[idx];

            if cell.is_black() {
                current_pattern = ((current_pattern << 1) ^ 1) & remap_pattern;
            } else {
                current_pattern = (current_pattern << 1) & remap_pattern;
            }

            if column >= 9 && current_pattern == pattern_mask || current_pattern == reverse_mask {
                subtotal += 40;
            }
        }

        subtotal
    }

    pub fn penalty_score_eval_three(&self, body: &Vec<Cell>) -> usize {
        let mut penalty_total = 0;

        for i in 0..self.size {
            penalty_total += self.check_column(body, i as isize);
            penalty_total += self.check_row(body, i as isize);
        }
        penalty_total
    }

    pub fn penalty_score_eval_one(&self, body: &Vec<Cell>) -> usize {
        let canvas_size = self.size;
        let mut current_row = 0;
        let mut consecutive_same_color = 1;
        let mut tracking_black = false;
        let mut penalty_total = 0;

        for cell in body {
            if cell.point.0 != current_row {
                // new row, so reset again
                if consecutive_same_color >= 5 {
                    penalty_total += consecutive_same_color - 2;
                }
                consecutive_same_color = 1;
                tracking_black = cell.is_black();
                current_row = cell.point.0;
                continue;
            }
            let is_black = cell.is_black();

            if (tracking_black && is_black) || (!tracking_black && !is_black) {
                consecutive_same_color += 1;
            } else if (tracking_black && !is_black) || (!tracking_black && is_black) {
                // tally up and reset
                if consecutive_same_color >= 5 {
                    penalty_total += consecutive_same_color - 2;
                }

                consecutive_same_color = 1;
                tracking_black = is_black;
            }
        }

        let cell_count = body.len();
        let mut idx = 0;

        while idx < cell_count {
            let cell = &body[idx];
            let is_black = cell.is_black();

            if (tracking_black && is_black) || (!tracking_black && !is_black) {
                consecutive_same_color += 1;
            } else if (tracking_black && !is_black) || (!tracking_black && is_black) {
                // tally up and reset
                if consecutive_same_color >= 5 {
                    penalty_total += consecutive_same_color - 2;
                }

                consecutive_same_color = 1;
                tracking_black = is_black;
            }

            if cell.point.1 == (canvas_size - 1) && cell.point.0 == (canvas_size - 1) {
                break;
            } else if cell.point.0 == canvas_size - 1 {
                idx = cell.point.1 + 1;
                consecutive_same_color = 1;
            } else {
                idx += canvas_size;
            }
        }

        penalty_total
    }

    pub fn penalty_score_eval_four(&self, body: &Vec<Cell>) -> usize {
        // total modules
        let total_modules = body.len() as f64;
        let black_modules: f64 = body.iter()
            .fold(0.0, |acc, ref c| {
                if c.is_black() {
                    acc + 1.0
                } else {
                    acc
                }
            });

        let black_percentage = ((black_modules / total_modules) * 100.0).round() as usize;
        let remainder = black_percentage % 5;

        let prev_mul = black_percentage - remainder;
        let next_mul = prev_mul + 5;

        let prev_abs = (50 - prev_mul as isize).abs();
        let next_abs = (50 - next_mul as isize).abs();

        let prev_div = prev_abs / 5;
        let next_div = next_abs / 5;


        if prev_div < next_div {
            (prev_div * 10) as usize
        } else {
            (next_div * 10) as usize
        }
    }

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
        println!("data {}, codewords {}", data.len(), codewords.len());
        println!("cw: {:?}", self.codeword_properties);
        // for (idx, byte) in codewords.iter().enumerate() {
        //     if let Some(data_byte) = data.get(idx + 2) {
        //         println!("{:b} original", 112 << 4);
        //     }
        //     println!("{:b} byte #: {} ", idx, byte);
        // }
    }

    pub fn encode_error_correction_codewords(&mut self) {
        let ecc_len = self.codeword_properties.ecc_codeword_count;
        let ecc_per_block = ecc_len / self.codeword_properties.block_count;
        let encoder = Encoder::new(ecc_per_block);
        let (group_one, group_two) = self.codeword_properties.get_data_cw_total_for_groups();
        let data_codewords = &mut self.codewords;
        let mid_point = group_one.blocks * group_one.codewords_per_block;
        let mut blocks: Vec<Buffer> = vec![];
        let mut data_section: Vec<u8> = vec![];

        {
            let (first_group_data, second_group_data) = data_codewords.split_at(mid_point);
            for data_block in first_group_data.chunks(group_one.codewords_per_block) {
                let buffer = encoder.encode(data_block);
                blocks.push(buffer);
            }

            if group_two.blocks > 0 {
                for data_block in second_group_data.chunks(group_two.codewords_per_block) {
                    let buffer = encoder.encode(data_block);
                    blocks.push(buffer);
                }
            }

            let codeword_max = if group_one.codewords_per_block > group_two.codewords_per_block {
                group_one.codewords_per_block
            } else {
                group_two.codewords_per_block
            };

            let mut interleaved_data = interleave_blocks(&blocks[..], codeword_max, ecc_per_block);
            data_section.append(&mut interleaved_data);
        }

        *data_codewords = data_section;
    }


    pub fn translate_data(&mut self) {
        let data_cw_length = self.codeword_properties.get_data_codeword_length();
        let data_length = self.data.len() as u16;
        let encoding = self.encoding;
        let copied_data = self.data.clone();
        {
            let codewords = &mut self.codewords;
            let mut first_byte = encoding << 4;
            let mut second_byte: u8 = data_length as u8;

            if self.version > 9 {
                second_byte = (data_length >> 8) as u8;
                codewords.push(first_byte | (second_byte >> 4));
                first_byte = second_byte << 4;
                second_byte = data_length as u8;
            } else {
                second_byte = data_length as u8;
            }

            /*
                what am I doing...

                1. if there's a 16 bit integer, it needs to be broken up|
                2. the first half of the 16 bit integer is used first
                3. cast it to an 8 bit integer
                4. bitwise operation normally with the mode byte
                
            */

            let mut index = 0;

            loop {
                codewords.push(first_byte | (second_byte >> 4));
                first_byte = second_byte << 4;

                if let Some(byte) = copied_data.get(index) {
                    second_byte = *byte;
                    index += 1;
                } else {
                    codewords.push(second_byte << 4);
                    break;
                }
            }
        }

        // pad the end of the message codewords, alternating between 17 and 236, until it fills the allotted amount for the version


        let mut swap = false;
        let mut n = 0;
        while self.codewords.len() < data_cw_length {
            n += 1;
            if swap {
                self.codewords.push(17u8);
            } else {
                self.codewords.push(236u8);
            }

            swap = !swap;
        }

        if self.debug_mode {
            println!("THIS IS HOW MANY PADDED BYTES ARE ADDED BEFORE INTERLEAVING {}", n);

            println!("after translate {}", self.codewords.len());
            for (idx, cw) in self.codewords.iter().enumerate() {
                println!("Codeword {}:  {:08b}", idx, cw);
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
                x * self.size + y,
                y * self.size + x
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
        let mut vertical: Point<usize> = Point(0, 8);
        let mut horizontal: Point<usize> = Point(8, 0);

        while horizontal.1 < self.size {
            let idx = (horizontal.0 * self.size) + horizontal.1;
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
            let idx = (vertical.0 * self.size) + vertical.1;
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
        let dark_module_coord: Point<usize> = Point((4 * self.version) + 9, 8);
        let idx = (dark_module_coord.0 * self.size) + dark_module_coord.1;
        match body.get_mut(idx) {
            Some(cell) => {
                cell.module_type = CellType::DarkModule;
                cell.color = Color { r: 0, g: 0, b: 0 };
            },
            None => {}
        }
    }

    pub fn apply_alignment_patterns(&self, body: &mut Vec<Cell>, points: &Vec<PlotPoint>) {
        for plot_point in points {
            let idx = (plot_point.point.0 * self.size) + plot_point.point.1;
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
            let pt: Point<usize> = Point(x, y);
            let idx = (pt.0 * self.size) + pt.1;
            match body.get_mut(idx) {
                Some(c) => {
                    c.module_type = CellType::Separator;
                    c.color = Color { r: 255, g: 255, b: 255 };
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

    pub fn apply_finder_patterns(&self, body: &mut Vec<Cell>, alignment_point: Point<usize>) {
        for plot_point in self.plot_spiral(&alignment_point, 6, 0) {
            let idx = (plot_point.point.0 * self.size) + plot_point.point.1;
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
            let pt: Point<usize> = Point(x, y);
            let idx = (pt.0 * self.size) + pt.1;
            match body.get_mut(idx) {
                Some(cell) => {
                    match cell.module_type {
                        CellType::None | CellType::Format => {
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
        let mut pts: Vec<usize> = vec![6];
        let mut n = 6;
        // let last_column = self.size - 7;
        let version_bracket = match self.version {
            1 => 0,
            2...6 => 1,
            7...13 => 2,
            14...21 => 3,
            22...28 => 4,
            29...36 => 5,
            37...41 => 6,
            _ => 0
        };



        let modifier = if version_bracket == 1 {
            self.size - 13
        } else {
            (self.size - 12) / version_bracket
        };

        for _ in 0..version_bracket {
            n += modifier;
            pts.push(n);
        }

        let pts: Vec<PlotPoint> = self.get_point_combinations(pts)
            .into_iter()
            .filter(|pt| {
                let idx = (pt.0 * self.size) + pt.1;
                let cell_ref = body.get(idx);
                if cell_ref.is_none() {
                    return false
                }

                let cell = cell_ref.unwrap();
                let result = match cell.module_type {
                    CellType::None => true,
                    _ => false
                };

                result
            })
            .flat_map(|pt| {
                self.plot_spiral(&pt, 4, 2)
            })
            .collect();

        pts
    }

    pub fn get_point_combinations(&self, numbers: Vec<usize>) -> Vec<Point<usize>> {
        let mut pairs: Vec<Point<usize>> = vec![]; //numbers.iter().map(|n| (*n, *n)).collect();
        let xnumbers: Vec<usize> = numbers.iter().cloned().collect();
        for n in numbers {
            for xn in xnumbers.iter() { // can I use the same vec inside its iteration?
                pairs.push(Point(n, *xn));
            }
        }
        pairs
    }

    pub fn plot_spiral(&self, origin_pt: &Point<usize>, size: usize, diff: usize) -> Vec<PlotPoint> {
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
