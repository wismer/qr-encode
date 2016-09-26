extern crate image;
pub mod grid;
use grid::message::{ErrorCorrectionLevel};
use grid::grid::{create_grid, encode_byte, Cell};
use grid::traverse::{Point};
use std::fs::File;
use std::path::Path;
use image::{
    GenericImage,
    ImageBuffer,
    Rgb,
    Pixel
};

fn draw_pixels(x: usize, y: usize) -> Vec<(u32, u32)> {
    let i = (x * 20) as u32;
    let j = (y * 20) as u32;
    let mut pixels: Vec<(u32, u32)> = vec![];
    for col in i..(i + 20) {
        for row in j..(j + 20) {
            pixels.push((col, row))
        }
    }

    pixels
}


fn main() {
    let mut qr = create_grid(49, 2);
    let message = String::from("www.wikipedia.org");
    let mut img = ImageBuffer::new(49 * 20, 49 * 20);
    for byte in message.into_bytes() {
        encode_byte(byte, &mut qr, 49 * 49);
    }
    let size = 49;
    //Iterate over all pixels in the image
    // [0, 0, 0] == black
    // [255, 255, 255] == white
    for cell in qr {
        match cell {
            Cell::Fixed(x, y) => {
                let pixels = draw_pixels(x, y);
                for pixel in pixels {
                    let (x, y) = pixel;
                    // println!("({x}, {y})", x=x, y=y);
                    img.put_pixel(x, y, Rgb { data: [0, 0, 0] });
                }
            },
            Cell::Content(point) => {
                let color: [u8; 3];
                if point.is_edge {
                    color = [120, 30, 50];
                } else if point.is_corner {
                    color = [30, 120, 50];
                } else {
                    color = [255, 255, 255];
                }
                let pixels = draw_pixels(point.x, point.y);
                for pixel in pixels {
                    let (x, y) = pixel;
                    // println!("({x}, {y})", x=x, y=y);
                    img.put_pixel(x, y, Rgb { data: color });
                }
            },
            _ => {}
        };


    }
    // for cell in qr.bits.iter() {
        // let ref point = get_point(cell);
        // let neighbors = get_neighboring_points
        // let row = point.x;
        // let col = point.y;
        // let i = (row * 20) as u32;
        // let j = (col * 20) as u32;
        // for y in i..(i + 20) {
        //     for x in j..(j + 20) {
        //         img.put_pixel(x, y, Rgb { data: point.color });
        //     }
        // }
    // }

    let ref mut fout = File::create(&Path::new("qr.png")).unwrap();
    let _ = image::ImageRgb8(img).save(fout, image::PNG);
}
