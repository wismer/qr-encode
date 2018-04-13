
pub mod qr_encoder;
extern crate image as image_lib;
extern crate reed_solomon;

use qr_encoder::qr::QR;
use qr_encoder::config::{QRConfig};
use qr_encoder::cell::{Cell, CellType, Color};
use qr_encoder::util::{get_pixel_points, args};
use qr_encoder::cursor::{Cursor, QRContext};

use std::fs::File;
use std::path::Path;

use self::image_lib::{
    ImageBuffer,
    Rgba
};


fn assign_bit_from_codeword(index: usize, body: &mut Vec<Cell>, dark: bool) -> isize {
    let cell = body.get_mut(index).unwrap();
    match cell.module_type {
        CellType::None => {
            if dark {
                cell.value = 1;
                cell.color = Color { r: 0, b: 0, g: 0 };
            } else {
                cell.value = 0;
                cell.color = Color { r: 255, g: 255, b: 255 };
            };
            cell.module_type = CellType::Message;

            -1
        },
        _ => 0
    }
}

fn zig_zag_points(canvas_size: usize) -> Vec<usize> {
    let mut col = canvas_size - 1;
    let mut row = canvas_size - 1;
    let mut indices = vec![];
    let mut inc = -1;
    while col != 0 {
        if col == 6 {
            col -= 1;
            continue;
        }
        for c in 0..2 {
            let index = (row * canvas_size) + col - c;
            indices.push(index);
        }
        
        if row == 0 && inc == -1 {
            inc = 1;
            col -= 2;
        } else if row == canvas_size - 1 && inc == 1 {
            inc = -1;
            if col == 1 {
                col -= 1;
            } else {
                col -= 2;
            }
        } else {
            row = (row as isize + inc) as usize;            
        }
    }
    
    indices
}

fn create_qr_image(qr: &QR, config: &QRConfig) {
    let dimensions: u32 = (config.size) as u32;
    let mut img = ImageBuffer::new(dimensions * 28, dimensions * 28);

    for pixel in img.pixels_mut() {
        let white: Rgba<u8> = Rgba { data: [255, 255, 255, 255] };
        *pixel = white;
    }

    for cell in &qr.body {
        for pixel in get_pixel_points(&cell) {
            let (x, y, color) = pixel;
            // if x % 20 == 0 || y % 20 == 0 {
                // // cell border
                // let rgb = Rgba { data: [125, 125, 125, 255] };
                // img.put_pixel(x, y, rgb);
            // } else {
            let rgb = Rgba { data: [color.r as u8, color.g as u8, color.b as u8, 255] };
            img.put_pixel(x, y, rgb);
            // }
        }
    }

    let ref mut fout = File::create(&Path::new("qr.png")).unwrap();
    let _ = image_lib::ImageRgba8(img).save(fout, image_lib::PNG);
}


fn main() {
    let mut config: QRConfig = args();
    let drawn_path: Vec<usize> = vec![8; 0];

    let mut qr: QR = QR {
        body: config.create_body(),
        cursor: Cursor {
            context: QRContext {
                free: 0,
                algn: 0,
                timing: 0,
                off: 0,
                msg: 0
            },
            drawn_path: drawn_path,
            current_index: config.size * config.size - 1
        },
    };

    config.verify_version(); // TODO
    config.translate_data();

    {
        let c = &mut config;
        c.encode_error_correction_codewords();
    }

    qr.setup(&config);

    {
        let mut bit_index = 7;
        let mut codeword_index = 0usize;
        let pathing = zig_zag_points(config.size);
        let pathing_iter = &mut pathing.iter();
        // codewords
        while codeword_index < config.codewords.len() {
            let cw = config.codewords[codeword_index];
            let idx = pathing_iter.next().unwrap();
            bit_index += assign_bit_from_codeword(*idx, &mut qr.body, (cw >> bit_index) & 1 == 1);

            if bit_index == -1 {
                bit_index = 7;
                codeword_index += 1;
            }
        }

        let mut remainder_bits = config.get_remainder_bit_length();
        while remainder_bits > 0 {
            let i = pathing_iter.next().unwrap();
            remainder_bits += assign_bit_from_codeword(*i, &mut qr.body, false);
        }
    }

    {
        let body = &mut qr.body;
        let mut best = 0;
        let mut best_pattern = 0;
        for pattern in 0..7 {
            let mut copy = &mut body.clone();
            config.apply_mask_pattern(&mut copy, pattern);
            let score = config.eval_penalty_scores(copy);
            if best == 0 || score < best {
                best = score;
                best_pattern = pattern;
            }
        }

        config.apply_mask_pattern(body, best_pattern);
        config.encode_format_areas(body, best_pattern as u8);
        
        if config.version >= 7 {
            config.apply_version_information(body);
        }


    }


    create_qr_image(&qr, &config);
}
