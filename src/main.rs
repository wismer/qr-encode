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
    let message = String::from("www.wikipedia.org");
    qr.encode(message, 0b0100);
    let mut img = ImageBuffer::new(49 * 20, 49 * 20);
    let size = 49;
    //Iterate over all pixels in the image
    for bit in qr.bits.iter() {
        let color = bit.color(size);
        let (row, col) = bit.coords(&size);
        let i = (row * 20) as u32;
        let j = (col * 20) as u32;
        for y in i..(i + 20) {
            for x in j..(j + 20) {
                img.put_pixel(x, y, Rgb { data: color });
            }
        }
    }

    let ref mut fout = File::create(&Path::new("qr.png")).unwrap();
    let _ = image::ImageRgb8(img).save(fout, image::PNG);
}