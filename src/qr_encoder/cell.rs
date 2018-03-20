use std::ops::{Shr, Add, Mul, Sub};
use std::fmt::Debug;
use std::cmp::PartialOrd;

#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub r: u32,
    pub g: u32,
    pub b: u32
}

#[derive(Debug, Clone)]
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


#[derive(Debug, Clone)]
pub struct Cell {
    pub module_type: CellType,
    pub value: u8,
    pub point: Point<usize>,
    pub color: Color
}

impl Cell {
    pub fn is_black(&self) -> bool {
        self.color.r == 0 && self.color.g == 0 && self.color.b == 0
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Point<T>(pub T, pub T);

impl Point<isize> {
    fn is_invalid(&self, canvas_size: isize) -> bool {
        self.0 < 0 && self.0 >= canvas_size && self.1 < 0 && self.1 >= canvas_size
    }

    // pub fn to(&self, destination_point: (Point<isize>, Point<isize>), canvas_size: isize) -> Option<Vec<isize>> {
    //     if destination_point.is_invalid(canvas_size) || self.is_invalid(canvas_size) {
    //         return None
    //     }
    //
    //     let mut points: Vec<isize> = vec![];
    //     let (modifier, steps) = match destination_point {
    //         Point(x, _) if x > self.0 => (canvas_size, x - self.0),
    //         Point(x, _) if x < self.0 => (-canvas_size, self.0 - x),
    //         Point(_, y) if y > self.1 => (1, y - self.1),
    //         Point(_, y) if y < self.1 => (-1, self.1 - y),
    //         _ => panic!("{:?} is not valid", destination_point)
    //     };
    //
    //     let mut idx = (self.0 * canvas_size) + self.1;
    //     for _ in 0..steps + 1 {
    //         points.push(idx);
    //         idx += modifier;
    //     }
    //
    //     Some(points)
    // }
}


#[derive(Debug)]
pub struct PlotPoint {
    pub point: Point<usize>,
    pub color: Color
}
