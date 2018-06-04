
pub mod qr_encoder;
extern crate image as image_lib;
extern crate reed_solomon;

use qr_encoder::qr::QR;
use qr_encoder::config::{QRConfig};
use qr_encoder::cell::{Cell, CellType, Color};
use qr_encoder::util::{get_pixel_points, args};

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
    let path = Path::new("qr.png");
    let ref mut fin = File::create(&path);
    match fin {
        Ok(_) => {
            image_lib::ImageRgba8(img).save(&path);
        },
        Err(err) => {
            panic!("{} ERROR", err);
        }
    }
}

fn main() {
    // parse the command line arguments
    let mut config: QRConfig = args();
    // kick off the encoding process
    let canvas = config.gen_qr_code();
    // generate the image from the encoded data
    create_qr_image(&canvas, &config);
}
