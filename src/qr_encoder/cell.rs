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


#[derive(Debug)]
pub struct PlotPoint {
    pub point: Point<usize>,
    pub color: Color
}
