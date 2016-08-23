extern crate image;
pub mod grid;
use grid::message::{ErrorCorrectionLevel};
use grid::grid::{QRGrid};
use std::fs::File;
use std::path::Path;
use image::{
    GenericImage,
    ImageBuffer,
    Rgb,
    Pixel
};

fn main() {
    let mut qr = QRGrid::new(49, 2, ErrorCorrectionLevel::Low);
    // let message = String::from("www.wikipedia.org");
    // qr.encode(message, 0b0100)

    //Construct a new ImageBuffer with the specified width and height.
    let mut img = ImageBuffer::new(49 * 20, 49 * 20);

    //Construct a new by repeated calls to the supplied closure.
    // let img = ImageBuffer::from_fn(512, 512, |x, y| {
    //     if x % 2 == 0 {
    //         image::Luma([0u8])
    //     } else {
    //         image::Luma([255u8])
    //     }
    // });
    // 1, 1 -> 1, 7
    //  |       |
    // 7, 1 -> 7, 7

    //Iterate over all pixels in the image
        // let mut pix = img.put_pixel(300, 300, Rgb { data: [3, 100, 255] });
        // println!("{:?}", pix);
    for bit in qr.bits.iter() {
        let color = bit.color();
        let i = (bit.x * 20) as u32;
        let j = (bit.y * 20) as u32;
        for y in i..(i + 20) {
            for x in j..(j + 20) {
                img.put_pixel(x, y, Rgb { data: color });
            }
        }
    }
    //     // for x in (bit.x)..(bit.x + 2) {
    //     //     for y in (bit.y)..(bit.y + 2) {
    //     //         img.put_pixel(x as u32, y as u32, Rgb { data: color })
    //     //     }
    //     // }
    // }
    // img.put_pixel(x, y, Rgb { data: color });
    let ref mut fout = File::create(&Path::new("fractal.png")).unwrap();
    let _ = image::ImageRgb8(img).save(fout, image::PNG);
}