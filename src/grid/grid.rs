use grid::message::{FormatInfo, ErrorCorrectionLevel};

enum QRSection {
    Fixed,
    FixedBridge,
    Format,
    Error,
    Message,
    MsgMode,
    MsgLength,
    None
}

struct BitPath {
    path: Vec<(usize, usize)>
}

enum Orientation {
    RotateUpward([usize; 8]),
    RotateDownward([usize; 8]),
    Downward([usize; 8]),
    Upward([usize; 8])
}

pub struct Bit {
    pub x: usize,
    pub y: usize,
    // true for 1, false for 0
    pub val: bool,
    section: QRSection
}

struct Chunk(u8, u8);

pub struct QRGrid {
    size: usize,
    pub bits: Vec<Bit>,
    format_info: FormatInfo
}


impl Bit {
    fn is_valid(&self) -> bool {
        match self.section {
            QRSection::None => true,
            _ => false
        }
    }

    pub fn color(&self) -> [u8; 3] {
        match self.section {
            QRSection::None => [255, 255, 255],
            QRSection::FixedBridge => {
                if self.x == 6 && self.y % 2 == 0 || self.y == 6 && self.x % 2 == 0 {
                    [0, 0, 0]
                } else {
                    [255, 255, 255]
                }
            },
            QRSection::Fixed => {
                if self.x == 1 || self.x == 5 || self.x == 49 - 6 || self.x == 47 {
                    match self.y {
                        1...5 | 43...47 => [255, 255, 255],
                        7 => [255, 255, 255],
                        _ => [0, 0, 0]
                    }
                } else if self.y == 1 || self.y == 5 || self.y == 49 - 6 || self.y == 47 {
                    match self.x {
                        1...5 | 43...47 => [255, 255, 255],
                        7 => [255, 255, 255],
                        _ => [0, 0, 0]
                    }
                } else if self.x == 7 || self.y == 7 {
                    [255, 255, 255]
                } else {
                    [0, 0, 0]
                }
            },
            QRSection::Format => [130, 0, 155],
            _ => [255, 255, 255]
        }
    }
}

fn is_fixed_area(x: usize, y: usize, size: usize) -> bool {
    x <= 7 && (y <= 7 || (size - y) <= 7) || y <= 7 && (size - x) <= 7
}

fn is_bridge_area(x: usize, y: usize, size: usize) -> bool {
    x == 6 && (y >= 8 && y <= size - 9) || x >= 8 && x <= (size - 9) && y == 6
}

fn is_format_area(x: usize, y: usize, size: usize) -> bool {
    if x == 8 {
        match y {
            0...8 | 42...48 => true,
            _ => false
        }
    } else if y == 8 {
        match x {
            0...8 | 42...48 => true,
            _ => false
        }
    } else {
        false
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
                bit = Bit { x: row, y: col, val: false, section: QRSection::Fixed };
            } else if is_bridge_area(row, col, size) {
                bit = Bit { x: row, y: col, val: false, section: QRSection::FixedBridge };
            } else if is_format_area(row, col, size) {
                bit = Bit { x: row, y: col, val: false, section: QRSection::Format };
            } else {
                bit = Bit { x: row, y: col, val: false, section: QRSection::None };
            }
            bits.push(bit);
        }

        let format_info = FormatInfo::new(mask, error_correction);
        QRGrid { size: size, bits: bits, format_info: format_info }
    }

    pub fn get_point_color(&self, x: u32, y: u32) -> [u8; 3] {
        let row = x;
        let col = y % 49;
        let index = row + col;
        let ref bit: Bit = self.bits[index as usize];
        if bit.is_valid() {
            [255, 255, 255]
        } else {
            [0, 0, 0]
        }
    }

    pub fn show(&self) {
        let f = self.format_info.mask_func_factory();
        for n in &self.bits {
            println!("{} {}", n.x, n.y);

            let v = f(n.x, n.y);
            println!("{}", v);
        }
    }

    pub fn encode(&mut self, message: String, mode: u8) {
        let mut bits = &mut self.bits;
        let mut payload: Vec<Chunk> = vec![Chunk(mode, 4)];
        let msg_length = message.len();
        payload.push(Chunk(msg_length as u8, 8));

        for byte in message.into_bytes() {
            payload.push(Chunk(byte, 8));
        }

        let mut start_x = self.size - 1;
        let mut start_y = self.size - 1;

        for b in payload {
            // I think I need to stick in a direction enum for Chunk
        }
    }

    fn is_empty_bit(&self, x: usize, y: usize) -> bool {
        if x < 0 || y < 0 || x >= self.size || y >= self.size {
            return false
        }

        let bit = &self.bits[x * (self.size - 1) + y];
        bit.is_valid()
    }

    fn get_valid_path(&self, x: usize, y: usize, block_size: usize) -> Option<BitPath> {
        let bit: Bit;
        let mut valid_path: Vec<(usize, usize)> = vec![];
        let length = valid_path.len();
        match length {
            4 | 8 => Some(BitPath { path: valid_path }),
            _ => None
        }
    }
}
