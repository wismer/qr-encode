use std::collections::HashMap;
use std::ops::{Add, Shl, Shr, Sub};
use grid::bit::Bit;


#[derive(Copy, Clone)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

impl Shl<usize> for Coord {
    type Output = Option<(usize, usize)>;

    fn shl(self, rhs: usize) -> Option<(usize, usize)> {
        match self.y {
             0 => None,
             _ => Some((self.x, self.y - rhs))
        }
    }
}

impl Shr<usize> for Coord {
    type Output = Option<(usize, usize)>;

    fn shr(self, rhs: usize) -> Option<(usize, usize)> {
        match self.y {
            48 => None,
             _ => Some((self.x, self.y + rhs ))
        }
    }
}

impl Add<usize> for Coord {
    type Output = Option<(usize, usize)>;

    fn add(self, rhs: usize) -> Option<(usize, usize)> {
        match self.x {
             0 => None,
             _ => Some((self.x - rhs, self.y))
        }
    }
}

impl Sub<usize> for Coord {
    type Output = Option<(usize, usize)>;

    fn sub(self, rhs: usize) -> Option<(usize, usize)> {
        match self.x {
            48 => None,
             _ => Some((self.x + rhs, self.y ))
        }
    }
}

impl Coord {
    fn index(&self, dim: usize) -> usize {
        (dim / self.x) + self.y
    }
}
