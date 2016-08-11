use grid::message::{FormatInfo, ErrorCorrectionLevel};

enum QRSection {
    Fixed,
    Format,
    Error,
    Message,
    MsgMode,
    MsgLength,
    None
}

enum Direction {
    Up,
    Down,
    Left,
    Right
}

pub struct Bit {
    x: usize,
    y: usize,
    // true for 1, false for 0
    val: bool,
    section: QRSection
}

struct Chunk(u8, u8);

pub struct QRGrid {
    size: usize,
    bits: Vec<Bit>,
    format_info: FormatInfo
}


fn is_fixed_area(x: usize, y: usize, size: usize) -> bool {
    x <= 7 && (y <= 7 || (size - y) <= 7) || y <= 7 && (size - x) <= 7
}

impl QRGrid {
    pub fn new(size: usize, mask: u8, error_correction: ErrorCorrectionLevel) -> QRGrid {
        let mut bits: Vec<Bit> = vec![];

        for i in 0..(size * size) {
            let row = i / size;
            let col = i % size;
            let mut bit: Bit;
            if is_fixed_area(row, col, size) {
                bit = Bit { x: row, y: col, val: false, section: QRSection::Fixed };
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

    fn set_chunk(&mut self) -> Direction {
        Direction::Right
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
}
