use std::env::{args_os};
use std::ffi::{OsStr};

use qr_encoder::cell::{Cell, Color};
use qr_encoder::config::{QRConfig, ECLevel, EncodingMode};

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

// fn content_length_for_version(version: usize, mode: ) -> usize {
//     match version {
//         1...10 => 8,
//     }
    
// }

fn get_ec_level(level: &str) -> ECLevel {
    match level {
        "l" => ECLevel::Low,
        "q" => ECLevel::Q,
        "h" => ECLevel::High,
        _ => ECLevel::Medium
    }
}

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
    let encoding = 8u8;
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
