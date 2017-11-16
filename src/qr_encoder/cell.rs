use std::ops::{Shr};

#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub r: u32,
    pub g: u32,
    pub b: u32
}

pub enum Direction {
    Up,
    Left,
    Down
}

#[derive(Debug)]
pub enum CellType {
    Finder,
    Alignment,
    Separator,
    Timing,
    DarkModule,
    VersionInformation,
    Format,
    Message,
    None
}


#[derive(Debug)]
pub struct Cell {
    pub module_type: CellType,
    pub value: u8,
    pub point: Point,
    pub color: Color
}

#[derive(Debug, Copy, Clone)]
pub struct Point(pub usize, pub usize);

impl Point {
    pub fn as_point(idx: usize, size: usize) -> Point {
        Point(idx / size, idx % size)
    }

    pub fn idx(&self, size: usize) -> usize {
        (self.0 * size) + self.1
    }
}

impl Shr<(isize, isize)> for Point {
    type Output = Point;

    fn shr(self, rhs: (isize, isize)) -> Point {
        let (rx, ry) = rhs;
        let x = (self.0 as isize) + rx;
        let y = (self.1 as isize) + ry;

        if x < 0 || y < 0 {
            Point(0, 0)
        } else {
            Point(x as usize, y as usize)
        }
    }
}

#[derive(Debug)]
pub struct PlotPoint {
    pub point: Point,
    pub color: Color
}
