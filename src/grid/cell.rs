use grid::util::*;
use grid::traverse::Point;
use grid::grid::Grid;

#[derive(Debug)]
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

#[derive(Debug)]
pub enum CellRef<'a> {
    Free(&'a Cell),
    Bridge(&'a Cell),
    Filled,
    Fixed,
    None
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
            x: x,
            y: y,
            paths: 0
        }
    }

    pub fn value(&self) -> CellRef {
        if self.is_bridge {
            CellRef::Bridge(self)
        } else if self.is_free() {
            CellRef::Free(self)
        } else {
            CellRef::None
        }
    }

    pub fn is_free(&self) -> bool {
        !self.is_fixed && self.is_empty && !self.is_format
    }

    pub fn as_point(&self) -> Point {
        Point { x: self.x, y: self.y }
    }
}

// trait Iterator {
//     type Item;
//     fn next(&mut self) -> Option<Self::Item>;
// }
//
// impl Iterator for Grid {
//     type Item = [Option<Cell>; 4];
//
//     fn next(&mut self) -> Option<Self::Item> {
//         let mut current_pt = self.current_pt;
//
//         // What would be None?
//
//         // What would be Some?
//     }
// }
