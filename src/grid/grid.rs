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

pub struct Bit {
    x: usize,
    y: usize,
    // true for 1, false for 0
    val: bool,
    section: QRSection
}

impl Bit {
    fn format(&mut self, size: usize) {
        match self.section {
            QRSection::Fixed => {

            }
        }
    }
}

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
        // fixed zones: 7x7 blocks for 21x21 (for now)
        //
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

    pub fn fill_fixed(&mut self) {
        
    }

    pub fn show(&self) {
        for n in &self.bits {
            println!("{} {}", n.x, n.y);
        }
    }

    pub fn encode(&mut self, message: String) {
        let bytes = message.into_bytes();

    }
}
