pub mod grid;

#[derive(Debug, Clone)]
struct CodePoint {
    x: u32,
    y: u32,
    is_bit: bool
}

struct QRSquare {
    size: u32,
    square: Vec<CodePoint>
}

struct QREncode {
    mode: u8,
    msg_lenth: u32,
    body: QRSquare
}

enum Tile {
    CodePoint { x: u32, y: u32, is_bit: bool },
    None
}

trait Setter {
    fn set(&mut self, index: usize, x: u32, y: u32);
    fn get(&self, index: usize) -> &CodePoint;
}

impl Setter for QRSquare {
    fn set(&mut self, index: usize, x: u32, y: u32) {
        self.square[index] = CodePoint { x: x, y: y, is_bit: false }
    }

    fn get(&self, index: usize) -> &CodePoint {
        &self.square[index]
    }
}

trait Draw {
    fn draw_fixed_point(&self);
}

impl Draw for QRSquare {
    fn draw_fixed_point(&self) {
        // let mut init_code_point = self.body[(3 * 21) + 3];
    }
}

fn main() {
    let grid = grid::grid::create_grid(21);
    // let mut qr = QREncode {
    //     mode: 1,
    //     msg_lenth: 32,
    //     body: QRSquare {
    //         square: vec![Tile; 441],
    //         size: 21
    //     }
    // };
    //
    // let mut index = 0usize;
    // for x in 0..qr.body.size {
    //     for y in 0..qr.body.size {
    //         qr.body.set(index, x, y);
    //         index += 1;
    //     }
    // }
    //
    // let code = qr.body.get(440);
    // qr.body.draw_fixed_point();
    // println!("y::{}, x::{}", code.y, code.x);
}