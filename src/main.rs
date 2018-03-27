
pub mod qr_encoder;
extern crate image as image_lib;
extern crate reed_solomon;

use qr_encoder::qr::QR;
use qr_encoder::config::{QRConfig};
use qr_encoder::util::{get_pixel_points, args};
use qr_encoder::cursor::{Cursor, QRContext};

use std::fs::File;
use std::path::Path;

use self::image_lib::{
    ImageBuffer,
    Rgba
};



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
        let data = &config.codewords[..];
        for byte in data.iter() {
            qr.encode_chunk(&byte, 8, &config);
        }

        let remainder_bits = &config.get_remainder_bit_length();
        qr.encode_chunk(&0, *remainder_bits, &config);
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
        println!("Best Pattern: {} score: {}", best_pattern, best);

        config.encode_format_areas(body, best_pattern as u8);
    }


    create_qr_image(&qr, &config);
}
