
pub mod qr_encoder;
extern crate image as image_lib;
extern crate reed_solomon;

use qr_encoder::qr::QR;
use qr_encoder::config::{QRConfig};
use qr_encoder::util::{codeword_info, get_pixel_points, square_count, args, CodeWord};
use qr_encoder::position::Position;

use std::fs::File;
use std::path::Path;

use self::image_lib::{
    ImageBuffer,
    Rgba
};
use self::reed_solomon::Encoder;


fn create_qr_image(qr: QR, config: &QRConfig) {
    let dimensions: u32 = (config.size) as u32;
    let mut img = ImageBuffer::new(dimensions * 20, dimensions * 20);
    for cell in qr.body {
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

    let start_point = (config.size * config.size) - 1;    
    let mut qr: QR = QR {
        body: config.create_body(),
        current_position: Position::new(start_point, config.size),
        previous_position: Position::new(start_point, config.size)
    };

    config.verify_version();
    config.translate_data();

    {
        let mut c = &mut config;
        c.encode_error_correction_codewords();
    }

    qr.setup(&config);
    println!("{:?}", config.codewords);

    {
        let data = &config.codewords[..];
        for byte in data.iter() {
            qr.encode_chunk(&byte, 8, &config);
        }
    }

    create_qr_image(qr, &config);
}
