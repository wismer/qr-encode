#[derive(Copy, Clone, Debug)]
pub struct Position {
    pub timing: u8,
    pub msg: u8,
    pub algn: u8,
    pub off: u8,
    pub free: u8,
    pub size: usize,
    pub current_index: usize,
    pub prev_index: usize
}

const UPPER_RIGHT: u8 = 1;
const LOWER_RIGHT: u8 = 2;
const LOWER_LEFT: u8 = 4;
const UPPER_LEFT: u8 = 8;
const TOP: u8 = UPPER_RIGHT | UPPER_LEFT;
const BOTTOM: u8 = LOWER_RIGHT | LOWER_LEFT;
const LEFT: u8 = LOWER_LEFT | UPPER_LEFT;
const RIGHT: u8 = LOWER_RIGHT | UPPER_RIGHT;
const UR_CORNER: u8 = RIGHT | UPPER_LEFT;
const LR_CORNER: u8 = RIGHT | LOWER_LEFT;
const LL_CORNER: u8 = LEFT | LOWER_RIGHT;
const UL_CORNER: u8 = LEFT | UPPER_RIGHT;
const ALL_CORNER: u8 = TOP | BOTTOM;

#[derive(Debug)]
enum Direction {
    Left,
    Up,
    UpRight,
    UpLeft,
    Down,
    DownLeft,
    DownRight,
    JumpLeft,
    AlignmentTimingBlock
}

enum DirectionEvent {
    SkipColumn,
    SkipRow,
    SkipAlignment,
    Ceiling,
    Floor,
    ZagUp,
    ZagDown,
    Up,
    Down
}

impl Position {
    pub fn new(start_index: usize, size: usize) -> Position {
        Position {
            free: 0,
            algn: 0,
            timing: 0,
            off: 0,
            msg: 0,
            size: size,
            current_index: start_index,
            prev_index: start_index
        }
    }

    fn guess_direction(&self, prev_area: Position, size: usize) -> Direction {
        if self.free & UPPER_RIGHT == UPPER_RIGHT && prev_area.free & RIGHT == 0 {
            Direction::UpRight
        } else if self.free & LOWER_RIGHT == LOWER_RIGHT && prev_area.free & RIGHT == 0 {
            Direction::DownRight
        } else if self.timing == TOP && prev_area.prev_index == self.current_index + 2 {
            Direction::JumpLeft
        } else if self.current_index % size == self.prev_index % size && self.current_index > self.prev_index {
            Direction::Down
        } else if self.current_index % size == self.prev_index % size && self.current_index < self.prev_index {
            Direction::Up
        } else {
            Direction::Left
        }
    }

    fn near_offside(&self, size: usize, prev_area: Position) -> usize {
        let row_diff = self.rows_from(prev_area.prev_index);

        match self.off {
            UR_CORNER | LR_CORNER => self.current_index - 1,
            RIGHT => self.current_index - 1,
            BOTTOM if self.free == TOP => self.current_index - size + 1,
            BOTTOM if self.free == UPPER_LEFT => self.current_index - 1,
            TOP if self.free == 0 => self.current_index - 1,
            TOP if self.free == LOWER_LEFT => self.current_index - 1,
            TOP if self.free == BOTTOM => self.current_index + size + 1,
            UPPER_RIGHT if self.free == ALL_CORNER ^ UPPER_RIGHT => self.current_index + 1 + size,
            UPPER_LEFT if self.free & LOWER_RIGHT == LOWER_RIGHT => self.current_index + 1 + size,
            // TOP => {
            //     if self.msg == LOWER_RIGHT {
            //         self.current_index - 1
            //     } else if row_diff == -2 || row_diff == 0 {
            //         self.current_index + size + 1
            //     } else {
            //         self.current_index - 1
            //     }
            // },
            _ => self.current_index - 1
        }
    }

    fn timing_offside(&self, size: usize, prev_area: Position) -> usize {
        // return self.current_index
        // let row_diff = self.rows_from(prev_area.prev_index);
        match self.off {
            RIGHT => self.current_index - 1,
            BOTTOM => {
                if prev_area.off == BOTTOM {
                    self.current_index - 1
                } else {
                    self.current_index - size + 1
                }
            },
            _ => self.current_index - 1
        }
        // match self.guess_direction(prev_area, size) {
        //     Direction::UpRight => self.current_index - size + 1,
        //     Direction::DownRight => self.current_index + size + 1,
        //     Direction::Down => self.current_index + size,
        //     Direction::Up => self.current_index - size,
        //     Direction::Left => {
        //         if self.current_index % size == 7 {
        //             self.current_index - 2 // left-hand timing pattern cross-over
        //         } else if self.current_index == self.prev_index - 1 {
        //             self.current_index + (size * 2) + 1
        //         } else {
        //             self.current_index - 1
        //         }
        //     }
        //     _ => self.current_index - 1
        // }
    }

    fn rows_from(&self, index: usize) -> isize {
        (self.current_index / self.size) as isize - (index / self.size) as isize
    }


    // Positive value denotes a "downward" trajectory, negative "upwards"
    fn columns_from(&self, index: usize) -> isize {
        (self.current_index / self.size) as isize - (index / self.size) as isize
    }

    fn near_timing(&self, prev_area: Position) -> usize {
        let row_diff = self.rows_from(prev_area.prev_index);
        let col_diff = self.columns_from(prev_area.prev_index);
        let size = self.size;
        let diff = self.current_index / size - prev_area.prev_index / size;
        panic!("fuck {}, {}", self.current_index, prev_area.current_index);
        match self.timing {
            TOP if self.current_index == prev_area.current_index => {
                panic!("well fuck everything {:?}, {:?}", self, prev_area);
                self.current_index - (size * 2) + 1
            },
            TOP if prev_area.prev_index == self.current_index - self.size * 2 => {
                self.current_index + size + 1
            },// guard against coming from above

            BOTTOM if prev_area.prev_index == self.current_index + self.size * 2 => self.current_index - size + 1,
            BOTTOM if self.current_index - 1 == prev_area.current_index => self.current_index + size + 1,
            _ => self.current_index - 1
        }
    }

    pub fn near_alignment(&self, size: usize, prev_area: Position) -> usize {
        match self.guess_direction(prev_area, size) {
            Direction::UpRight => {
                if self.algn == 0b1001 {
                    self.current_index - (size * 6) + 1
                } else if self.algn & 1 == 1 {
                    self.current_index - size
                } else if self.algn == 0b0110 || self.free == 0b1101 {
                    self.current_index - size + 1
                } else {
                    self.current_index - 1
                }
            },

            Direction::DownRight => {
                if self.free == 0b0110 {
                    self.current_index + size + 1
                } else if self.algn == 0b0110 {
                    self.current_index + (size * 6) + 1
                } else if self.algn == 0b1100 {
                    self.current_index + size + 1
                } else if self.algn & 0b0010 == 0b0010 {
                    self.current_index + size
                } else if self.algn & 0b0100 == 0b0100 {
                    self.current_index + size + 1
                } else if prev_area.timing > 0 {
                    self.current_index + size + 1
                } else {
                    self.current_index - 1
                }
            },

            Direction::Up => {
                if self.free == 0b1100 {
                    self.current_index - size
                } else if self.free == 0b1101 {
                    self.current_index - size + 1
                } else {
                    self.current_index - 1
                }
            },

            Direction::Left => {
                // if the previous position was to the right, and there's a single
                if self.current_index % size == self.prev_index % size {
                    if self.current_index > self.prev_index {
                        self.current_index + size
                    } else {
                        self.current_index - size
                    }
                } else if self.current_index + 1 == self.prev_index && self.algn == TOP {
                    self.prev_index - (size * 6)
                } else if self.current_index + 1 == self.prev_index && self.algn == BOTTOM {
                    self.prev_index + (size * 6)
                } else if self.current_index + 1 == self.prev_index && self.algn == UPPER_RIGHT {
                    self.current_index - size
                } else if self.current_index + 1 == self.prev_index && self.algn == LOWER_RIGHT {
                    self.current_index + (size * 6) + 1
                } else if self.current_index + 1 == self.prev_index && prev_area.timing == LOWER_RIGHT {
                    self.current_index + size + 1
                } else {
                    self.current_index - 1
                }
            },
            _ => self.current_index - 1
        }
    }

    pub fn print_binary(&self, size: usize, prev_area: Position) {
        println!("Direction: {:?}", self.guess_direction(prev_area, size));
        println!("Point: {}, {}", self.current_index / size, self.current_index % size);
        println!("Alignment {:b}", self.algn);
        println!("Offsides  {:b}", self.off);
        println!("Free      {:b}", self.free);
        println!("Message   {:b}", self.msg);
        println!("Timing    {:b}", self.timing);
        println!("Previous  {}", self.prev_index);
        println!("Current   {}", self.current_index);
    }

    pub fn row(&self) -> usize {
        self.current_index / self.size
    }

    pub fn column(&self) -> usize {
        self.current_index % self.size
    }

    fn timing_alignment(&self, size: usize, prev_area: Position) -> usize {
        self.current_index - 1
    }

    pub fn near_edge(&self, size: usize, prev_area: Position) -> usize {
        match self.guess_direction(prev_area, size) {
            Direction::DownRight => {
                if self.off == 0b0110 {
                    self.current_index - 1
                } else {
                    self.current_index + size + 1
                }
            },

            Direction::Up => {
                // if (self.prev_index / size) - (self.current_index / size) > 5 {
                self.current_index - size
            },

            Direction::Down => {
                self.current_index + size
            },

            Direction::UpRight => {
                if self.off == RIGHT || self.off == LEFT {
                    self.current_index - 1
                } else {
                    self.current_index - size + 1
                }
            },

            Direction::Left => {
                if self.free & 1 == 1 {
                    self.current_index - size + 1
                } else if self.free == 0b1110 {
                    self.current_index + size + 1
                } else if prev_area.timing == LEFT {
                    self.current_index - 1
                } else if prev_area.timing > 0 && self.off == BOTTOM {
                    self.current_index - 1
                } else if self.off == 0b1110 {
                    self.current_index - (size * 8) - 1
                } else if self.off == TOP && self.algn == LOWER_LEFT {
                    self.current_index - 2
                } else {
                    self.current_index - 1
                }
            },

            _ => self.current_index - 1
        }
    }

    pub fn adjust_position(mut self, size: usize, prev_area: Position) -> Position {
        self.print_binary(size, prev_area);
        let former_index = self.current_index;

        self.current_index = if self.timing > 0 && self.algn > 0 {
            self.timing_alignment(size, prev_area)
        } else if self.timing > 0 && self.off > 0 {
            self.timing_offside(size, prev_area)
        } else if self.timing > 0 {
            self.near_timing(prev_area)
        } else if self.off > 0 {
            self.near_offside(size, prev_area)
        } else if self.algn > 0 {
            self.near_alignment(size, prev_area)
        } else if self.msg == 0b0010 {
            self.current_index - size + 1
        } else if self.msg == 0b0001 {
            self.current_index + size + 1
        } else {
            self.current_index - 1
        };

        self.prev_index = former_index;
        if self.current_index == prev_area.current_index {
            panic!("fuck {}, {}", self.current_index, prev_area.current_index);
        }

        self
    }
}
