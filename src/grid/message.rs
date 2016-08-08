use grid::grid::Bit;

enum MessageChunks {
    Format(FormatInfo),
    ErrorCorrection(ErrorCorrectChunk),
    Message(MessageChunk),
    None
}

pub enum ErrorCorrectionLevel {
    Low,
    Medium,
    High,
    Quartile
}

enum MaskPatterns {
    Vertical,
    Diagonal,
    Checkered,
    Horizontal,
    Amorphous,
    Other // hoo boy this is hard stuff. TODO.
}

/*
    ex:
        let start_point = 0;
        let mut tiles: Tiles = &mut self.tiles;
        let mut i = 0usize;
        match self.pattern {
            MaskPatterns::Vertical => {
                while i < 8 {
                    tiles.set(|&tile|
                        if tile.x % i == 0 {
                            tile.bit ^= 1;
                        }
                    )
                }
            },
            etc... TODO
        }

*/

enum BitPattern {
    Up,
    Down,
    Left,
    Right
}

/*
    ex: ( I think )
        let pattern: [u8; 8] = match pattern_enum_field {
            BitPattern::Up => [1, 0, 2, 3, 4, 5, 6, 7],
            BitPattern::Down => [6, 7, 5, 4, 3, 2, 0, 1],
            BitPattern::Left => [5, 4, 3, 2, 7, 6, 1, 0],
            BitPattern::Right => [7, 6, 1, 0, 5, 4, 3, 2]
        }
*/

struct Generator;
struct ErrorCorrectChunk; // this needs to be fleshed out later
struct MessageChunk {
    bits: [Bit; 8],
    pattern: BitPattern
}

pub struct FormatInfo {
    level: ErrorCorrectionLevel,
    mask: u8,
    error_correction: Option<u16>
}

fn lcm(x: u8, y: u8) -> u8 {
    (x * y) / gcd(x, y)
}

fn gcd(x: u8, y: u8) -> u8 {
    let (mut a, mut b): (u8, u8) = (x, y);
    while a % b != 0 {
        let rem = a % b;
        a = b;
        b = rem;
    }

    b
}

impl Generator {
    fn mask_format_info(&self, mask: u16) -> Option<u16> {
        let mut x = mask;
        let gen = 0b10100110111;
        for n in 0..5 {
            if x & (1 << (n + 10)) == 1 {
                x ^= gen << n;
            }
        }

        Some(x)
    }
}

impl FormatInfo {
    pub fn new(mask: u8, level: ErrorCorrectionLevel) -> FormatInfo {
        FormatInfo { mask: mask, level: level, error_correction: None }
    }

    pub fn encode_formatting(&mut self) {
        let error_level: u8 = match self.level {
            ErrorCorrectionLevel::Low => 3,
            ErrorCorrectionLevel::Medium => 2,
            ErrorCorrectionLevel::Quartile => 1,
            ErrorCorrectionLevel::High => 0
        };
        let format_info = (error_level << 3) ^ self.mask;

        let generator = Generator;
        self.error_correction = generator.mask_format_info(format_info as u16);
    }
}
