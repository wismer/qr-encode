// #![allow(dead_code)]

// pub mod qr_encoder;

// use qr_encoder::util::{codeword_info, square_count};
// use qr_encoder::config::{ECLevel, QRConfig, EncodingMode, ecc_format};
// use qr_encoder::cell::CellType;
// use qr_encoder::qr::QR;


// fn qr_config_with_general_opts(version: usize, ec_level: ECLevel, encoding_mode: EncodingMode, message: Vec<u8>) -> QRConfig {
//     QRConfig {
//         version: version,
//         data: message,
//         encoding: 0,
//         codewords: vec![],
//         codeword_properties: codeword_info(version, &ec_level),
//         mask: 4,
//         encoding_mode: encoding_mode,
//         debug_mode: false,
//         requires_alignment: version > 6,
//         finder_points: [
//             (0, 0),
//             ((square_count(version) - 7), 0),
//             (0, (square_count(version) - 7))
//         ],
//         size: (((version - 1) * 4) + 21),
//         err_correction_level: ec_level
//     }
// }

// fn create_qr(config: &QRConfig) -> QR {
//     QR {
//         body: config.create_body() }
// }

// #[cfg(test)]
// mod tests {
//     // use super::qr_encoder::util::{args};
//     // use super::qr_encoder::qr::{QROptions, QR};

//     // #[test]
//     // fn test_qr_create() {
//     //     let qr_opt = args();
//     // }

//     #[test]
//     fn test_ecc_version_info() {
//         use super::qr_encoder::config::ecc_format;
//         let gen_poly = 7973;
//         let mut version = 7;

//         let mut expected_outcome = 0b000111110010010100u32;
//         let mut actual_outcome = ecc_format::<u32>(version, gen_poly, None);

//         assert_eq!(expected_outcome, actual_outcome);

//         version = 8;
//         expected_outcome = 0b1000010110111100u32;
//         actual_outcome = ecc_format::<u32>(version, gen_poly, None);

//         assert_eq!(expected_outcome, actual_outcome);

//         version = 33;
//         expected_outcome = 0b100001011011110000u32;
//         actual_outcome = ecc_format::<u32>(version, gen_poly, None);

//         assert_eq!(expected_outcome, actual_outcome);
//     }

//     #[test]
//     fn test_ecc_format() {
//         use super::*;


//         let message = String::from("Hello, World!").into_bytes();
//         let mut qr_opts = qr_config_with_general_opts(7, ECLevel::Low, EncodingMode::Byte, message);
//         let mut qr = create_qr(&qr_opts);
//         qr.setup(&mut qr_opts);
//         // qr.encode_data(&qr_opts);
//         qr_opts.encode_format_areas(&mut qr.body, 4u8);

//         // assert_eq!(4, qr_opts.mask);
//         let mut index = qr_opts.size * 8;
//         let mut actual_format_bytestring = 0;
//         let expected_format_bytestring = 0b110011000101111;
//         let mut pos = 14;

//         while index >= 8 {

//             let cell = &qr.body[index];
//             match cell.module_type {
//                 CellType::Format => {
//                     if cell.is_black() {
//                         actual_format_bytestring |= (1 << pos);   
//                     }
//                     pos -= 1;
//                 },
//                 _ => {}
//             }

//             if index == 8 {
//                 break;
//             }

//             if index % qr_opts.size == 8 {
//                 index -= qr_opts.size;
//             } else {
//                 index += 1;
//             }
//         }
        
//         assert_eq!(expected_format_bytestring, actual_format_bytestring);
//     }
// }
