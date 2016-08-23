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

struct Point<'a> {
    bit: &'a Bit
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
            QRSection::None => {
                if self.val {
                    [0, 0, 0]
                } else {
                    [255, 255, 255]
                }
            },
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
        let msg_length = message.len();
        let mut index = 2401;
        for byte in message.into_bytes() {
            let mut i = 7;
            while i > 0 {
                let ref mut bit = bits[index - 1];
                let xbit = byte & (1 << i);
                bit.val = xbit == 0;
                i -= 1;
                index -= 1;
            }
        }
    }

    fn point_within_bounds(&self, index: usize) -> bool {
        index < (self.size * 2) - 1 && index > 0
    }

    fn get_possible_paths(&self, index: usize) {

    }

    fn get_point(&self, x: usize, y: usize) -> Option<Point> {
        if x > (self.size - 1) * 2 || y > (self.size - 1) * 2 {
            return None
        }

        let index = (x * (self.size - 1)) + y;
        let ref bit = self.bits[index];
        let point = Point { bit: bit };
        Some(point)
    }
}
