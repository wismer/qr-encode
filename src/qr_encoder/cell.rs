use std::ops::{Shr, Add};

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

impl Cell {
    pub fn apply_mask(&mut self, mask: usize) {
        let Point(row, col) = self.point;

        if (row + col) % 2 == 0 {
            match self.value {
                1 => self.color = Color { r: 255, g: 255, b: 255 }, // change to black
                _ => self.color = Color { r: 0, g: 0, b: 0 }
            }
        }
    }

    pub fn is_black(&self) -> bool {
        self.color.r == 0 && self.color.g == 0 && self.color.b == 0
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Point(pub usize, pub usize);

impl Point {
    pub fn as_point(idx: usize, size: usize) -> Point {
        Point(idx / size, idx % size)
    }

    pub fn to(self, destination_point: Point, canvas_size: isize) -> Vec<isize> {

        let mut points = vec![];
        let (modifier, steps) = match destination_point {
            Point(x, _) if x > self.0 => (canvas_size, x - self.0),
            Point(x, _) if x < self.0 => (-canvas_size, self.0 - x),
            Point(_, y) if y > self.1 => (1, y - self.1),
            Point(_, y) if y < self.1 => (-1, self.1 - y),
            _ => panic!("{:?} is not valid", destination_point)
        };
        // println!("from: {:?} to: {:?}, {:?}", self, destination_point, modifier);

        let mut idx = self.idx(canvas_size as usize) as isize;

        for _ in 0..steps {
            // println!("{:?}", idx);
            points.push(idx);
            idx += modifier;
        }

        points
    }

    pub fn idx(&self, size: usize) -> usize {
        (self.0 * size) + self.1
    }

    pub fn move_to(&self, rhs: (isize, isize)) -> Point {
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

impl Add<(usize, usize)> for Point {
    type Output = Point;

    fn add(self, rhs: (usize, usize)) -> Point {
        if rhs.0 == 0 {
            Point(self.0, self.1 + rhs.1)
        } else {
            Point(self.0 + rhs.0, self.1)
        }
    }
}

#[derive(Debug)]
pub struct PlotPoint {
    pub point: Point,
    pub color: Color
}

impl Iterator for Point {
    type Item = Point;


    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
