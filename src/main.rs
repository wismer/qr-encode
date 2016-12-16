pub mod grid;
use grid::message::{ErrorCorrectionLevel};
use grid::grid::{create_grid, encode_byte, Grid};
use grid::cell::Cell;
use grid::traverse::Point;

fn main() {
    let qr_version = 1;
    let size = 49;
    let message = String::from("www.wikipedia.org - here you can find junk and stuff and whatever and some things of greater importance i just want a longer byte length please thanks");

    create_grid(size, 2, qr_version, message);
}
