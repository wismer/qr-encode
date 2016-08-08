pub mod grid;
use grid::message::{ErrorCorrectionLevel};
use grid::grid::{QRGrid};

fn main() {
    let mut qr = QRGrid::new(21, 2, ErrorCorrectionLevel::Low);
    qr.show();
}