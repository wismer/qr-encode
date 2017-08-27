use std::ops::{Add, Sub, Mul, Div};

#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub r: u32,
    pub g: u32,
    pub b: u32
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
    Free,
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

pub enum CellFlow {
    OutOfBounds,
    Unavailable,
    Available(usize)
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

    pub fn off_edge(&self, future_position: (isize, isize), size: usize) -> bool {
        let (fx, fy) = future_position;
        if self.0 == size && fx == 1 {
            true
        } else if self.0 == 0 && fx == -1 {
            true
        } else if self.1 == size && fy == 1 {
            true
        } else if self.1 == 0 && fy == -1 {
            true
        } else {
            false
        }
    }
}

impl Sub<usize> for Point {
    type Output = Point;

    fn sub(self, rhs: usize) -> Point {
        Point(self.0, self.1 - rhs)
    }
}

impl Add<usize> for Point {
    type Output = Point;

    fn add(self, rhs: usize) -> Point {
        Point(self.0, self.1 + rhs)
    }
}

impl Mul<usize> for Point {
    type Output = Point;

    fn mul(self, rhs: usize) -> Point {
        Point(self.0 + 1, self.1)
    }
}

impl Div<usize> for Point {
    type Output = Point;

    fn div(self, rhs: usize) -> Point {
        Point(self.0 - 1, self.1)
    }
}

#[derive(Debug)]
pub struct PlotPoint {
    pub point: Point,
    pub color: Color
}
