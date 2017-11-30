use std::env::{args_os};
use std::ffi::{OsStr};

use qr_encoder::cell::{Cell, Color};
use qr_encoder::config::{QRConfig, ECLevel, EncodingMode};


// L M Q H
#[derive(Copy, Clone, Debug)]
pub struct ECCodeWordCount(usize, usize, usize, usize);

// L M Q H
#[derive(Copy, Clone, Debug)]
pub struct CodeWordBlock(usize, usize, usize, usize);

#[derive(Debug)]
pub struct CodeWord {
    pub ecc_codeword_count: usize,
    pub block_count: usize,
    pub capacity: usize
}

const CODEWORD_COUNT: [usize; 41] = [
    0, 26, 44, 70, 100, 134, 172, 196, 242, 292, 346,
    404, 466, 532, 581, 655, 733, 815, 901, 991, 1085,
    1156, 1258, 1364, 1474, 1588, 1706, 1828, 1921, 2051, 2185,
    2323, 2465, 2611, 2761, 2876, 3034, 3196, 3362, 3532, 3706
];

const CODEWORD_BLOCKS: [CodeWordBlock; 40] = [
    // directly copied from https://github.com/soldair/node-qrcode/blob/master/lib/core/error-correction-code.js
    CodeWordBlock(1, 1, 1, 1),
    CodeWordBlock(1, 1, 1, 1),
    CodeWordBlock(1, 1, 2, 2),
    CodeWordBlock(1, 2, 2, 4),
    CodeWordBlock(1, 2, 4, 4),
    CodeWordBlock(2, 4, 4, 4),
    CodeWordBlock(2, 4, 6, 5),
    CodeWordBlock(2, 4, 6, 6),
    CodeWordBlock(2, 5, 8, 8),
    CodeWordBlock(4, 5, 8, 8),
    CodeWordBlock(4, 5, 8, 11),
    CodeWordBlock(4, 8, 10, 11),
    CodeWordBlock(4, 9, 12, 16),
    CodeWordBlock(4, 9, 16, 16),
    CodeWordBlock(6, 10, 12, 18),
    CodeWordBlock(6, 10, 17, 16),
    CodeWordBlock(6, 11, 16, 19),
    CodeWordBlock(6, 13, 18, 21),
    CodeWordBlock(7, 14, 21, 25),
    CodeWordBlock(8, 16, 20, 25),
    CodeWordBlock(8, 17, 23, 25),
    CodeWordBlock(9, 17, 23, 34),
    CodeWordBlock(9, 18, 25, 30),
    CodeWordBlock(10, 20, 27, 32),
    CodeWordBlock(12, 21, 29, 35),
    CodeWordBlock(12, 23, 34, 37),
    CodeWordBlock(12, 25, 34, 40),
    CodeWordBlock(13, 26, 35, 42),
    CodeWordBlock(14, 28, 38, 45),
    CodeWordBlock(15, 29, 40, 48),
    CodeWordBlock(16, 31, 43, 51),
    CodeWordBlock(17, 33, 45, 54),
    CodeWordBlock(18, 35, 48, 57),
    CodeWordBlock(19, 37, 51, 60),
    CodeWordBlock(19, 38, 53, 63),
    CodeWordBlock(20, 40, 56, 66),
    CodeWordBlock(21, 43, 59, 70),
    CodeWordBlock(22, 45, 62, 74),
    CodeWordBlock(24, 47, 65, 77),
    CodeWordBlock(25, 49, 68, 81)
];

const EC_CODEWORD_TABLE: [ECCodeWordCount; 40] = [
    ECCodeWordCount(7, 10, 13, 17),
    ECCodeWordCount(10, 16, 22, 28),
    ECCodeWordCount(15, 26, 36, 44),
    ECCodeWordCount(20, 36, 52, 64),
    ECCodeWordCount(26, 48, 72, 88),
    ECCodeWordCount(36, 64, 96, 112),
    ECCodeWordCount(40, 72, 108, 130),
    ECCodeWordCount(48, 88, 132, 156),
    ECCodeWordCount(60, 110, 160, 192),
    ECCodeWordCount(72, 130, 192, 224),
    ECCodeWordCount(80, 150, 224, 264),
    ECCodeWordCount(96, 176, 260, 308),
    ECCodeWordCount(104, 198, 288, 352),
    ECCodeWordCount(120, 216, 320, 384),
    ECCodeWordCount(132, 240, 360, 432),
    ECCodeWordCount(144, 280, 408, 480),
    ECCodeWordCount(168, 308, 448, 532),
    ECCodeWordCount(180, 338, 504, 588),
    ECCodeWordCount(196, 364, 546, 650),
    ECCodeWordCount(224, 416, 600, 700),
    ECCodeWordCount(224, 442, 644, 750),
    ECCodeWordCount(252, 476, 690, 816),
    ECCodeWordCount(270, 504, 750, 900),
    ECCodeWordCount(300, 560, 810, 960),
    ECCodeWordCount(312, 588, 870, 1050),
    ECCodeWordCount(336, 644, 952, 1110),
    ECCodeWordCount(360, 700, 1020, 1200),
    ECCodeWordCount(390, 728, 1050, 1260),
    ECCodeWordCount(420, 784, 1140, 1350),
    ECCodeWordCount(450, 812, 1200, 1440),
    ECCodeWordCount(480, 868, 1290, 1530),
    ECCodeWordCount(510, 924, 1350, 1620),
    ECCodeWordCount(540, 980, 1440, 1710),
    ECCodeWordCount(570, 1036, 1530, 1800),
    ECCodeWordCount(570, 1064, 1590, 1890),
    ECCodeWordCount(600, 1120, 1680, 1980),
    ECCodeWordCount(630, 1204, 1770, 2100),
    ECCodeWordCount(660, 1260, 1860, 2220),
    ECCodeWordCount(720, 1316, 1950, 2310),
    ECCodeWordCount(750, 1372, 2040, 2430)
];

pub fn codeword_info(version: usize, err_correction_level: &ECLevel) -> CodeWord {
    let ecc_settings: ECCodeWordCount = EC_CODEWORD_TABLE[version - 1];
    let block_count = CODEWORD_BLOCKS[version - 1];
    let capacity = CODEWORD_COUNT[version];

    let (ec_cw_count, blocks): (usize, usize) = match err_correction_level {
        &ECLevel::Low => (ecc_settings.0, block_count.0),
        &ECLevel::Medium => (ecc_settings.1, block_count.1),
        &ECLevel::Q => (ecc_settings.2, block_count.2),
        &ECLevel::High => (ecc_settings.3, block_count.3)
    };

    CodeWord {
        ecc_codeword_count: ec_cw_count,
        capacity: capacity,
        block_count: blocks
    }
}


pub fn get_pixel_points(cell: &Cell) -> Vec<(u32, u32, Color)> {
    let i = (cell.point.0 * 20) as u32;
    let j = (cell.point.1 * 20) as u32;
    let mut pixels: Vec<(u32, u32, Color)> = vec![];
    for row in i..(i + 20) {
        for col in j..(j + 20) {
            pixels.push((col, row, Color { g: cell.color.g, b: cell.color.b, r: cell.color.r }));
        }
    }

    pixels
}

pub fn square_count(version: usize) -> usize {
    (((version - 1) * 4) + 21)
}

pub fn set_color(index: usize) -> Color {
    // temporarily color the cells as a kind of debugging
    match index {
        0 => Color { r: 255, g: 120, b: 16 },
        1 => Color { r: 205, g: 120, b: 16 },
        2 => Color { r: 155, g: 120, b: 16 },
        3 => Color { r: 105, g: 120, b: 16 },
        4 => Color { r: 55, g: 120, b: 16 },
        5 => Color { r: 5, g: 120, b: 16 },
        6 => Color { r: 255, g: 175, b: 16 },
        7 => Color { r: 0, g: 0, b: 0 },
        _ => Color { r: 255, g: 255, b: 0 }
    }
}

fn get_content_length(mode: u8, version: usize) -> usize {
    let modifier = match version {
        1...10 => 0,
        11...27 => 2,
        _ => 4
    };

    match mode {
        1 => 10 + modifier,
        2 => 9 + modifier,
        8 => 12 + modifier,
        _ => {
            if version < 10 {
                8
            } else {
                16
            }
        }
    }
}

fn get_ec_level(level: &str) -> ECLevel {
    match level {
        "l" => ECLevel::Low,
        "q" => ECLevel::Q,
        "h" => ECLevel::High,
        _ => ECLevel::Medium
    }
}

// fn get_best_version_for_content(cw: &CodeWord, data: &Vec<u8>, mode: u8, requested_version: usize) -> usize {
//     // TODO: make the `request_version` an Option type
//     // make a copy of the data
//     // something to figure out....
//     // do I do the padding first, so it's evenly divisible by 8, or figure out the mode first?


//     let mut content = data.clone();
//     content.insert(0, mode);

//     let content_length = get_content_length(mode, requested_version) + 4; // 4bits (mode) + (x)bits content length
//     let rem = content_length % 8;
//     if rem != 0 {
//         match content.last_mut() {
//             Some(n) => n = n << rem,
//             None => {}
//         }
//     }


// }

// fn split_int(left: u8, right: u8) -> ()

// fn convert_data_into_codewords(cw: &CodeWord, data: &Vec<u8>, mode: u8, version: usize) -> Vec<u8> {
//     // first pad the combined bits of mode, content length, and data.
//     let data_length = data.len();
//     let content_length = get_content_length(mode, version);
//     let mut codewords: Vec<u8> = vec![];

//     codewords.push(mode);

//     if content_length > 8 {
//         // as a 16 bit integer, AND it
//         let first_part = (data_length as u16).rotate_right(8);
//         let second_part = data_length as u16;
//         codewords.push(first_part as u8);
//         codewords.push(second_part as u8);
//     } else {
// 
//     }
// 
// }

pub fn args() -> QRConfig {
    /*
        default options are....
            if no version, the default version is 21


        to do:
            flag for encoding type - default will be utf-8 (i think?)
            ???

    */
    let mut qr_args = args_os();
    let mut version = 14usize;
    let mut data: Option<Vec<u8>> = None;
    let mut ec_level: ECLevel = ECLevel::Medium;
    let encoding = 4u8;
    let mut arg = qr_args.next();

    while arg.is_some() {
        let value = arg.unwrap();
        if value == OsStr::new("-v") {
            version = match qr_args.next() {
                Some(n) => {
                    let x = n.to_str().unwrap().parse::<usize>();
                    match x {
                        Ok(nx) if nx < 81 => nx, // if it fails to parse, or parses a number greater than 81, set it to version 21.
                        Ok(_) => 21usize,
                        Err(_) => 21usize
                    }
                },
                None => 21usize
            }
        } else if value == OsStr::new("-m") {
            data = match qr_args.next() {
                Some(msg) => {
                    let string = String::from(msg.to_str().unwrap());
                    Some(string.into_bytes())
                },
                None => panic!("sdasd")
            }
        } else if value == OsStr::new("-ec") {
            ec_level = match qr_args.next() {
                Some(ec) => {
                    let ec = ec.to_str().unwrap();
                    get_ec_level(&ec)
                },
                None => ECLevel::Medium
            }
        }


        arg = qr_args.next();
    }

    QRConfig {
        version: version,
        data: data.unwrap(),
        codewords: vec![],
        codeword_properties: codeword_info(version, &ec_level),
        encoding: 4u8,
        encoding_mode: EncodingMode::Byte,
        requires_alignment: version > 1,
        err_correction_level: ec_level,
        size: (((version - 1) * 4) + 21),
        finder_points: [
            (0, 0),
            ((square_count(version) - 7), 0),
            (0, (square_count(version) - 7))
        ]
    }
}
