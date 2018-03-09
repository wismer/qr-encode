
pub mod qr_encoder;
extern crate image as image_lib;
extern crate reed_solomon;

use qr_encoder::qr::QR;
use qr_encoder::config::{QRConfig};
use qr_encoder::util::{codeword_info, get_pixel_points, square_count, args, CodeWord};
use qr_encoder::position::Position;
use qr_encoder::cell::{Point, Color, CellType};
use qr_encoder::cursor::{Cursor, QRContext};

use std::fs::File;
use std::path::Path;

use self::image_lib::{
    ImageBuffer,
    Rgba
};
use self::reed_solomon::Encoder;


fn create_qr_image(qr: &QR, config: &QRConfig) {
    let dimensions: u32 = (config.size) as u32;
    let mut img = ImageBuffer::new(dimensions * 20, dimensions * 20);
    for cell in &qr.body {
        for pixel in get_pixel_points(&cell) {
            let (x, y, color) = pixel;
            if x % 20 == 0 || y % 20 == 0 {
                // cell border
                let rgb = Rgba { data: [125, 125, 125, 255] };
                img.put_pixel(x, y, rgb);
            } else {
                let rgb = Rgba { data: [color.r as u8, color.g as u8, color.b as u8, 255] };
                img.put_pixel(x, y, rgb);
            }
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
    }
    let mut penalty_total = 0;
    let first = config.penalty_score_eval_one(&qr.body);
    println!("1: {:?}", first);
    let second = config.penalty_score_eval_two(&qr.body);
    println!("2: {:?}", second);
    let third = config.penalty_score_eval_three(&qr.body);
    println!("3: {:?}", third);
    let fourth = config.penalty_score_eval_four(&qr.body);
    println!("4: {:?}", fourth);

    {
        let qrbody = &mut qr.body;
        for cell in qrbody {
            match cell.module_type {
                CellType::Message => cell.apply_mask(config.mask),
                _ => {}
            }
        }
    }

    create_qr_image(&qr, &config);
}
