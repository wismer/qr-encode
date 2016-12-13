extern crate image as image_lib;

use std::fs::File;
use std::path::Path;

use grid::cell::Cell;
use grid::grid::Grid;
use self::image_lib::{
    ImageBuffer,
    Rgb,
    Frame
};

struct Color {
    r: u8,
    g: u8,
    b: u8
}


fn get_pixel_points(cell: &Cell) -> Vec<(u32, u32, Color)> {
    let i = (cell.x * 20) as u32;
    let j = (cell.y * 20) as u32;
    let mut pixels: Vec<(u32, u32, Color)> = vec![];
    let mut x_border = true;
    let mut y_border = true;
    for col in i..(i + 20) {
        x_border = match col % 20 {
            2...18 => false,
            _ => true
        };
        for row in j..(j + 20) {
            y_border = match row % 20 {
                2...18 => false,
                _ => true
            };
            let color: Color;
            let shader = match cell.paths {
                0 => 30,
                1 => 45,
                3 => 90,
                4 => 120,
                _ => 0
            };
            if x_border || y_border {
                color = Color { r: 125, b: 125, g: 125 };
            } else if cell.is_fixed || cell.is_bit {
                color = Color { r: 0, b: 0, g: 0 + shader };
            } else if cell.is_bridge {
                color = Color { r: 200, b: 100, g: 10 + shader };
            } else if cell.is_format {
                color = Color { r: 10, b: 200, g: 100 + shader };
            } else {
                color = Color { r: 255 - shader, b: 255 - shader, g: 255 - shader };
            }
            pixels.push((col, row, color));
        }
    }

    pixels
}

pub fn create_qr_image(grid: &Grid) {
    let mut img = ImageBuffer::new(49 * 20, 49 * 20);
    for row in grid.rows {
        for cell in row.cells {
            for pixel in get_pixel_points(&cell) {
                let (x, y, color) = pixel;
                let rgb = Rgb { data: [color.r, color.g, color.b] };
                img.put_pixel(x, y, rgb);
            }
        }
    }
    let ref mut fout = File::create(&Path::new("qr.png")).unwrap();
    let _ = image_lib::ImageRgb8(img).save(fout, image_lib::PNG);
}
