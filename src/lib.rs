pub mod qr_encoder;


#[cfg(test)]
mod tests {
    use super::qr_encoder::util::{args};
    use super::qr_encoder::qr::{QROptions, QR};

    #[test]
    fn test_qr_create() {
        let qr_opt = args();
    }
}
