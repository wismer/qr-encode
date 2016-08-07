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


    struct ErrorCorrection(u8);

    struct FormatInfo {
        level_correction: ErrorCorrectionLevel,
        mask: u8,
        error_correction: Option<u32>
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

    impl FormatInfo {
        fn new(mask: u8, level: ErrorCorrectionLevel) -> FormatInfo {
            FormatInfo { mask: mask, level_correction: level, error_correction: None }
        }

        fn apply_error_correction(&mut self) {
            let error_level: u8 = match self.level_correction {
                ErrorCorrectionLevel::Low => 3,
                ErrorCorrectionLevel::Medium => 2,
                ErrorCorrectionLevel::High => 0,
                ErrorCorrectionLevel::Quartile => 1
            };

            let mask = self.mask;

            let lcm =  // least common multiple of error and mask
        }
    }

    fn make_format() {
        let mut format_info: FormatInfo = FormatInfo {
            level_correction: ErrorCorrectionLevel::Low,
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