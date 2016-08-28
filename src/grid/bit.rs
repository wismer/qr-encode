use grid::grid::{QRSection};

pub struct Bit {
    pub idx: usize,
    pub val: bool,
    pub filled: bool,
    pub section: QRSection
}

pub struct BitRef<'a> {
    bit: &'a Bit
}

impl Bit {
    pub fn is_valid(&self) -> bool {
        match self.section {
            QRSection::ContentBody => true,
            _ => false
        }
    }

    pub fn coords(&self, size: &usize) -> (usize, usize) {
        (self.idx / size, self.idx % size)
    }

    pub fn color(&self, size: usize) -> [u8; 3] {
        let x = self.idx / size;
        let y = self.idx % size;
        match self.section {
            QRSection::ContentBody => {
                if self.val {
                    [0, 0, 0]
                } else {
                    [255, 255, 255]
                }
            },
            QRSection::FixedBridge => {
                if x == 6 && y % 2 == 0 || y == 6 && x % 2 == 0 {
                    [0, 0, 0]
                } else {
                    [255, 255, 255]
                }
            },
            QRSection::Fixed => {
                if x == 1 || x == 5 || x == 49 - 6 || x == 47 {
                    match y {
                        1...5 | 43...47 => [255, 255, 255],
                        7 => [255, 255, 255],
                        _ => [0, 0, 0]
                    }
                } else if y == 1 || y == 5 || y == 49 - 6 || y == 47 {
                    match x {
                        1...5 | 43...47 => [255, 255, 255],
                        7 => [255, 255, 255],
                        _ => [0, 0, 0]
                    }
                } else if x == 7 || y == 7 {
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
