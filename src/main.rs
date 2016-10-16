extern crate image;
pub mod grid;
use grid::message::{ErrorCorrectionLevel};
use grid::grid::{create_grid, encode_byte, Grid};
use grid::cell::Cell;
use grid::traverse::Point;
use std::fs::File;
use std::path::Path;
use image::{
    GenericImage,
    ImageBuffer,
    Rgb,
    Pixel
};

enum ColorSet {
    Border(Color),
    CellFixed(Color),
    CellFree(Color)
}

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


fn find_next_point(grid: &Grid, point: &Point) -> Point {
    Point { x: 1, y: 1 }
}

    /*
    Keep changing my mind....


    ok....

    so....

    Priority should always be to check rightmost first.

    O - O - O
    |   |   |
    O -[0]- 0
    |   |   |
    O - 0 - 0

    if the above is true, and the bits are being aligned upward,
    then I know that -1, +1 is the next point to fill. But how is that detrmined?

    if the rightmost cell is filled...
        1. check the cell above and below the one to the right...
        2. if top-right is empty, and bottom-right is not, then that may hint
           what the alignment is - in this case going bottom to top.

    */

fn main() {
    let qr_version = 1;
    let mut qr = create_grid(49, 2, qr_version);
    let mut size = 49 * 49;
    let message = String::from("www.wikipedia.org - here you can find junk and stuff and whatever");
    let mut img = ImageBuffer::new(49 * 20, 49 * 20);
    let mut starting_point = size - 1;
    for i in 0..(size - 1) {
        qr.update_cell_paths(Point { x: i / 49, y: i % 49 });
    }
    let mut pt = Point { x: starting_point / 49, y: starting_point % 49 };
    for byte in message.into_bytes() {
        for i in 1..7 {
            let xbit = byte & (1 << i);
            let next_point = qr.get_next_valid_point(&pt);
            match next_point {
                Some(p) => {
                    println!("x={x}, y={y}, NEXT", x=pt.x, y=pt.y);
                    qr.encode_bit(xbit == 0, p);
                    pt = p;
                },
                None => println!("NOTHING")
            }
        }
    }
    let size = 49;
    for row in qr.rows {
        for cell in row.cells {
            for pixel in get_pixel_points(&cell) {
                let (x, y, color) = pixel;
                let rgb = Rgb { data: [color.r, color.g, color.b] };
                img.put_pixel(x, y, rgb);
            }
        }
    }

    let ref mut fout = File::create(&Path::new("qr.png")).unwrap();
    let _ = image::ImageRgb8(img).save(fout, image::PNG);
}
