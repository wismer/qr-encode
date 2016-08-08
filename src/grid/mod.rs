pub mod message;
pub mod grid {
    struct Bit {
        x: usize,
        y: usize,
        bit: bool // true for 1, false for 0
    }

    pub struct QRGrid {
        size: usize,
        bits: Vec<Bit>
    }

    impl QRGrid {
        fn new(size: usize) -> QRGrid {
            let mut bits: Vec<Bit> = vec![];
            for i in 0..(size * size) {
                let row = i / size;
                let col = i % size;

                bits.push(Bit { x: row, y: col, bit: false });
            }
            QRGrid { size: size, bits: bits }
        }
    }

    pub fn create_grid(size: usize) -> QRGrid {
        QRGrid::new(size)
    }
}
