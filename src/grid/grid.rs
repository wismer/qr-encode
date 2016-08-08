use grid::message::{FormatInfo, ErrorCorrectionLevel};

pub struct Bit {
    x: usize,
    y: usize,
    bit: bool // true for 1, false for 0
}

pub struct QRGrid {
    size: usize,
    bits: Vec<Bit>,
    format_info: FormatInfo
}

impl QRGrid {
    pub fn new(size: usize, mask: u8, error_correction: ErrorCorrectionLevel) -> QRGrid {
        let mut bits: Vec<Bit> = vec![];
        for i in 0..(size * size) {
            let row = i / size;
            let col = i % size;

            bits.push(Bit { x: row, y: col, bit: false });
        }
        let format_info = FormatInfo::new(mask, error_correction);
        QRGrid { size: size, bits: bits, format_info: format_info }
    }

    fn set_masking(&mut self) {

    }

    pub fn show(&self) {
        for n in &self.bits {
            println!("{} {}", n.x, n.y);
        }
    }
}
