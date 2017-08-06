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

#[derive(Debug)]
pub struct Point(pub usize, pub usize);

impl Point {
    pub fn as_point(idx: usize, size: usize) -> Point {
        Point(idx / size, idx % size)
    }

    pub fn idx(&self, size: usize) -> usize {
        (self.0 * size) + self.1
    }
}


#[derive(Debug)]
pub struct PlotPoint {
    pub point: Point,
    pub color: Color
}
