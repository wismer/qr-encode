
pub mod qr_encoder;
extern crate image as image_lib;
use qr_encoder::qr::{QROptions, QR};
use qr_encoder::util::{get_pixel_points, square_count, args};
use qr_encoder::area::Area;

use std::fs::File;
use std::path::Path;

use self::image_lib::{
    ImageBuffer,
    Rgba
};


fn create_qr_image(qr: QR) {
    let dimensions: u32 = (qr.config.size) as u32;
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
    let opts: QROptions = args();
    let mut qr: QR = QR {
        body: opts.create_body(),
        config: opts
    };
    qr.setup();

    let sample = "".to_string();
    // let sample = "abcdefhijklmnopqrstuvwxyzabcdefghabcdefhijabcdefhijabcdefhij".to_string();
    let area = Area {
        free: 0,
        msg: 0,
        off: 0,
        algn: 0,
        timing: 0,
        prev_index: 0,
        current_index: 0
    };
    let start_point = (qr.config.size * qr.config.size) - 1;
    let mut position: (usize, usize, Area) = qr.encode_chunk(10, 4, (start_point, start_point, area));

    let mut character_position = 0;
    // for s in sample.into_bytes().into_iter() {
    for s in 0..543 {
        println!("Character: {}, position: {}", s, character_position);
        position = qr.encode_chunk(s as u8, 8, position);
        character_position += 1;
    }

    create_qr_image(qr);
}
