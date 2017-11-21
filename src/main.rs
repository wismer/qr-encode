
pub mod qr_encoder;
extern crate image as image_lib;
extern crate reed_solomon;


use qr_encoder::qr::QR;
use qr_encoder::config::{QRConfig};
use qr_encoder::util::{get_pixel_points, square_count, args};
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
    let config: QRConfig = args();
    let start_point = (config.size * config.size) - 1;    
    let mut qr: QR = QR {
        body: config.create_body(),
        current_position: Position::new(start_point),
        previous_position: Position::new(start_point)
    };
    

    qr.setup(&config);
    qr.encode_meta(&config);

    {
        let data = &config.data;
        for byte in data.into_iter() {
            qr.encode_chunk(byte, 8, &config);
        }
    }

    create_qr_image(qr, &config);

    // let error_correction = Encoder::new(sample.len());
    // let scrambled_data = error_correction.encode(&sample.into_bytes());
    // let start_point = (qr.config.size * qr.config.size) - 1;
    // let mut position: (usize, usize, Area) = qr.encode_chunk(qr.config.encoding, 4, (start_point, start_point, area));


    // println!(" ------ {} ------", scrambled_data.len());
    // for s in sample.into_bytes().into_iter() {
    // // for s in 0..580 {
    //     println!("Character: {}, position: {}", s, character_position);
    //     position = qr.encode_chunk(s as u8, 8, position);
    //     character_position += 1;
    // }

    // qr.encode_chunk(10, 10, position);

    // create_qr_image(qr);
}
