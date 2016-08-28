use std::collections::HashMap;
use std::ops::{Add, Shl, Shr, Sub};
use grid::bit::Bit;

pub struct Point {
    pub x: usize,
    pub y: usize
}

impl Shl<usize> for Point {
    type Output = Point;

    fn shl(self, rhs: usize) -> Point {
        Point { x: self.x, y: self.y - rhs }
    }
}

impl Shr<usize> for Point {
    type Output = Point;

    fn shr(self, rhs: usize) -> Point {
        Point { x: self.x, y: self.y + rhs }
    }
}

impl Add<usize> for Point {
    type Output = Point;

    fn add(self, rhs: usize) -> Point {
        Point { x: self.x - rhs, y: self.y }
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Point {
        Point { x: self.x + 1, y: self.y }
    }
}

impl Point {
    fn index(&self, dim: usize) -> usize {
        (dim / self.x) + self.y
    }
}