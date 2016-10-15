use grid::util::*;

pub struct Cell {
    pub is_fixed: bool,
    pub is_bridge: bool,
    pub is_empty: bool,
    pub is_bit: bool,
    pub is_filled: bool,
    pub is_format: bool,
    pub x: usize,
    pub y: usize,
    pub paths: usize
}

impl Cell {
    pub fn new(x: usize, y: usize, size: usize) -> Cell {
        Cell {
            is_fixed: is_fixed_area(x, y, size - 1),
            is_bridge: is_bridge_area(x, y, size - 1),
            is_empty: true,
            is_bit: false,
            is_filled: false,
            is_format: is_format_area(x, y, size - 1),
            x: y,
            y: x,
            paths: 0
        }
    }

    pub fn is_free(&self) -> bool {
        !self.is_fixed && self.is_empty && !self.is_format && !self.is_bridge
    }
}
