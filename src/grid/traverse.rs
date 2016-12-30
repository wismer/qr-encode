use std::ops::{Add, Shl, Shr, Sub, BitXor};

pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Copy, Clone)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

pub struct Line {
    from: Point,
    to: Point,
    direction: Direction
}

impl Line {
    pub fn points(&self, blocks: usize) -> Vec<Point> {
        let mut points: Vec<Point> = vec![];
        match self.direction {
            Direction::Up => {
                for i in self.from.x..blocks {
                    points.push(Point { x: self.from.x - i, y: self.from.y });
                }
            },
            Direction::Left => {
                for i in self.from.y..blocks {
                    points.push(Point { x: self.from.x, y: self.from.y - i });
                }
            },
            Direction::Down => {
                for i in self.from.x..blocks {
                    points.push(Point { x: self.from.x + i, y: self.from.y });
                }
            },
            Direction::Right => {
                for i in self.from.y..blocks {
                    points.push(Point { x: self.from.x, y: self.from.y + i });
                }
            }
        }
        points
    }
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

impl BitXor for Point {
    type Output = bool;

    fn bitxor(self, rhs: Point) -> bool {
        // check to see if the current point is below the other point
        self.y >= rhs.y && self.x > rhs.x
    }
}

impl Shr<(isize, isize)> for Point {
    type Output = Option<Point>;

    fn shr(self, rhs: (isize, isize)) -> Option<Point> {
        let (x, y) = rhs;
        let (cx, cy) = (self.x as isize, self.y as isize);

        if cx + x < 0 || cy + y < 0 {
            return None
        }

        let point = Point { x: (cx + x) as usize, y: (cy + y) as usize };

        if point.x > 48 || point.y > 48 {
            return None
        }

        Some(point)
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
    pub fn square_points(start: Point, blocks: usize) -> Vec<Point> {
        let mut points: Vec<Point> = vec![start];
        for operator in ">-<+".chars() {
            Point::line(&mut points, blocks, operator);
        }

        points
    }

    pub fn line(points: &mut Vec<Point>, blocks: usize, operator: char) {
        let mut pt = *points.last().unwrap();
        for _ in 1..blocks {
            pt = match operator {
                '+' => (pt + 1).unwrap(),
                '-' => (pt - 1).unwrap(),
                '>' => (pt >> 1).unwrap(),
                '<' => (pt << 1).unwrap(),
                _ => panic!("missing the operator, my dude.")
            };
            points.push(pt);
        }
    }

    pub fn generate_adjacent_points(point: Point) -> Vec<Point> {
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
}
