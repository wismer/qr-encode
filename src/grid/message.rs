pub mod message {
    enum ErrorCorrectionLevel {
        Low,
        Medium,
        High,
        Quartile
    }

    enum ErrorCorrectionBody<T> {
        Some(T),
        None
    }

    struct Generator;
    struct ErrorCorrection(u8);

    struct FormatInfo {
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
            let gen = 0b10100110111;
            for n in 0..5 {
                if mask & (1 << (n + 10)) == 0 {
                    mask ^= gen << n;
                }
            }

            Some(mask)
        }
    }

    impl FormatInfo {
        fn new(mask: u8, level: ErrorCorrectionLevel) -> FormatInfo {
            FormatInfo { mask: mask, level: level, error_correction: None }
        }

        fn apply(&mut self) {
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

    fn make_format() {
        let mut format_info: FormatInfo = FormatInfo {
            level: ErrorCorrectionLevel::Low,
            mask: 0b0100,
            error_correction: None
        };
    }

    enum EncodeTypes {
        Error { len: u8, body: u32 },
        Message { len: u8, body: u32 },

    }

    struct Message {

    }
}