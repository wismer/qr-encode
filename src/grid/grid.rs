use grid::message::{FormatInfo, ErrorCorrectionLevel};
use std::collections::HashMap;
use grid::traverse::Point;
use grid::bit::{Bit};
use grid::util::*;

pub enum QRSection {
    Fixed,
    FixedBridge,
    Format,
    ContentBody,
    EncType,
    MetaData
}

struct Routes {
    leftward: Option<usize>,
    rightward: Option<usize>,
    forward: Option<usize>,
    backward: Option<usize>,
    upper_right: Option<usize>,
    upper_left: Option<usize>,
    lower_left: Option<usize>,
    lower_right: Option<usize>
}

pub struct QRGrid {
    size: usize,
    pub bits: Vec<Bit>,
    index: usize,
    format_info: FormatInfo
}

impl Routes {
    fn next(&self) -> usize {
        if self.leftward.is_some() && self.forward.is_some() && self.upper_right.is_some() {
            self.upper_right.unwrap()
        } else if self.leftward.is_some() {
            self.leftward.unwrap()
        } else {
            0
        }
    }

    fn available_paths(&self) -> usize {
        let mut count = 0;

        if self.leftward.is_some() {
            count += 1;
        }

        if self.rightward.is_some() {
            count += 1;
        }

        if self.forward.is_some() {
            count += 1;
        }

        if self.backward.is_some() {
            count += 1;
        }

        count
    }
}

impl QRGrid {
    pub fn new(size: usize, mask: u8, error_correction: ErrorCorrectionLevel) -> QRGrid {
        let mut bits: Vec<Bit> = vec![];
        for i in 0..(size * size) {
            let row = i / size;
            let col = i % size;
            let bit: Bit;
            if is_fixed_area(row, col, size) {
                bit = Bit { idx: i, val: false, filled: true, section: QRSection::Fixed };
            } else if is_bridge_area(row, col, size) {
                bit = Bit { idx: i, val: false, filled: true, section: QRSection::FixedBridge };
            } else if is_format_area(row, col, size) {
                bit = Bit { idx: i, val: false, filled: false, section: QRSection::Format };
            } else {
                bit = Bit { idx: i, val: false, filled: false, section: QRSection::ContentBody };
            }
            bits.push(bit);
        }

        let format_info = FormatInfo::new(mask, error_correction);
        QRGrid { size: size, bits: bits, format_info: format_info, index: (size * size) - 1 }
    }

    fn get(&self, next: isize) -> Option<usize> {
        let index = next + self.index as isize;
        if index >= (self.size * self.size) as isize || index < 0 {
            return None
        }

        let ref bit = self.bits[index as usize];
        if bit.is_valid() {
            Some(bit.idx)
         } else {
            None
        }
    }

    fn get_next_valid_path(&mut self) {
        let paths = self.get_paths();
        self.index = paths.next();
    }

    fn encode_chunk(&mut self, byte: u8, bit_count: isize) {
        let mut x = 0;
        let mut y = 0;
        let mut i = bit_count;
        let mut row_modifier = self.size;
        let mut rotate = false;
        while i >= 0 {
            let ref mut bit = self.bits[self.index];

            if !bit.is_valid() {
                break;
            } else {
                println!("{}, {}", self.index, "WHAT");
            }

            {
                let xbit = byte & (1 << i);
                let mask_bit = self.format_info.mask_func_factory();
                let (x, y) = bit.coords(&self.size);
                bit.val = mask_bit(x, y, xbit == 0);
                bit.filled = true;
                i -= 1;
            }
            self.get_next_valid_path();
        }
    }


    fn get_paths<'a>(&self) -> Routes {
        let size = self.size as isize;
        Routes {
            leftward: self.get(-1),
            rightward: self.get(1),
            backward: self.get(size),
            forward: self.get(size * -1),
            upper_right: self.get((size * -1) + 1),
            upper_left: self.get((size * -1) - 1),
            lower_left: self.get((size * 1) - 1),
            lower_right: self.get((size * 1) + 1)
        }
    }

    fn point_available(&self, point: usize) -> bool {
        let ref bit = self.bits[point];
        bit.is_valid()
    }

    fn point_within_bounds(&self, point: usize) -> bool {
        point < (self.size * 2) - 1 && point > 0
    }
}