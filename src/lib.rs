pub mod qr_encoder;


#[cfg(test)]
mod tests {
    // use super::qr_encoder::util::{args};
    // use super::qr_encoder::qr::{QROptions, QR};

    // #[test]
    // fn test_qr_create() {
    //     let qr_opt = args();
    // }

    #[test]
    fn test_ecc_version_info() {
        use super::qr_encoder::config::ecc_format;
        let gen_poly = 7973;
        let mut version = 7;

        let mut expected_outcome = 0b000111110010010100u32;
        let mut actual_outcome = ecc_format::<u32>(version, gen_poly, None);

        assert_eq!(expected_outcome, actual_outcome);

        version = 8;
        expected_outcome = 0b1000010110111100u32;
        actual_outcome = ecc_format::<u32>(version, gen_poly, None);

        assert_eq!(expected_outcome, actual_outcome);

        version = 33;
        expected_outcome = 0b100001011011110000u32;
        actual_outcome = ecc_format::<u32>(version, gen_poly, None);

        assert_eq!(expected_outcome, actual_outcome);
    }
}
