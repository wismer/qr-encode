
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

    qr.setup(&mut config);
    qr.encode_data(&config);

    create_qr_image(&qr, &config);
}
