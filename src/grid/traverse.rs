use std::collections::HashMap;
use std::ops::{Add, Shl, Shr, Sub};
use grid::bit::Bit;


#[derive(Copy, Clone)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Shl<usize> for Point {
    type Output = Option<Point>;

    fn shl(self, rhs: usize) -> Option<Point> {
        match self.y {
             0 => None,
             _ => Some(Point { x: self.x, y: self.y - rhs })
        }
    }
}

impl Shr<usize> for Point {
    type Output = Option<Point>;

    fn shr(self, rhs: usize) -> Option<Point> {
        match self.y {
            48 => None,
             _ => Some(Point { x: self.x, y: self.y + rhs })
        }
    }
}

impl Add<usize> for Point {
    type Output = Option<Point>;

    fn add(self, rhs: usize) -> Option<Point> {
        match self.x {
             0 => None,
             _ => Some(Point { x: self.x - rhs, y: self.y })
        }
    }
}

impl Sub<usize> for Point {
    type Output = Option<Point>;

    fn sub(self, rhs: usize) -> Option<Point> {
        match self.x {
            48 => None,
             _ => Some(Point { x: self.x + rhs, y: self.y })
        }
    }
}

impl Point {
    pub fn generate_adjacent_points(point: Point) -> Vec<Point> {
        let size = 49 * 49;
        let shift_operators = "< > - +";
        let mut points: Vec<Point> = vec![];
        for c in shift_operators.chars() {
            let pt: Option<Point> = match c {
                '>' => point << 1,
                '<' => point >> 1,
                '+' => point +  1,
                '-' => point -  1,
                 _  => None
            };

            match pt {
                Some(point) => points.push(point),
                None => {}
            }
        }

        points
    }

    fn index(&self, dim: usize) -> usize {
        (dim / self.x) + self.y
    }
}
