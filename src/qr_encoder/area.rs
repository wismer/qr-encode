#[derive(Copy, Clone)]
pub struct Area {
    pub timing: u8,
    pub msg: u8,
    pub algn: u8,
    pub off: u8,
    pub free: u8,
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

#[derive(Debug)]
enum Direction {
    Left,
    Up,
    UpRight,    
    UpLeft,
    Down,
    DownLeft,
    DownRight,
    JumpLeft
}

impl Area {
    fn get_direction(&self) -> Direction {
        let same_row: bool = self.same_row();
        let difference = if self.prev_index > self.current_index {
            self.prev_index - self.current_index
        } else {
            self.current_index - self.prev_index
        };

        if same_row && self.msg & 0b0001 == 1 || self.free == 0b0110 && self.timing == 0 {
            Direction::DownRight
        } else if same_row && self.msg & 0b0010 == 2 || self.free == 0b1001 || self.free == 0b1101 {
            Direction::UpRight
        } else if !same_row && self.prev_index > self.current_index && self.free & 0b1100 > 0 {
            Direction::Up
        } else if !same_row && self.prev_index < self.current_index && self.free == 0b0010 {
            Direction::Down
        } else if same_row && self.timing == 0b1001 {
            Direction::UpLeft
        } else {
            Direction::Left
        }
    }

    fn guess_direction(&self, prev_area: Area, size: usize) -> Direction {
        if self.free & UPPER_RIGHT == UPPER_RIGHT && prev_area.free & RIGHT == 0 {
            Direction::UpRight
        } else if self.free & LOWER_RIGHT == LOWER_RIGHT && prev_area.free & RIGHT == 0 {
            Direction::DownRight
        } else if self.timing == TOP && prev_area.prev_index != self.current_index + size {
            Direction::JumpLeft
        } else {
            Direction::Left
        }
    }

    // fn for_those_weird_cases(&self, size: usize, prev_area: Area) -> usize {
        
    // }

    pub fn near_alignment(&self, size: usize, prev_area: Area) -> usize {
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
                } else {
                    self.current_index - 1
                }
            },
            _ => self.current_index - 1
        }
    }

    pub fn print_binary(&self, size: usize, prev_area: Area) {
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

    pub fn same_row(&self) -> bool {
        self.current_index + 1 == self.prev_index
    }

    pub fn same_column(&self, size: usize) -> bool {
        self.current_index % size == self.prev_index % size
    }

    pub fn near_edge(&self, size: usize, prev_area: Area) -> usize {
        match self.guess_direction(prev_area, size) {
            Direction::DownRight => {
                if self.off == 0b0110 {
                    self.current_index - 1
                } else {
                    self.current_index + size + 1
                }
            },

            Direction::Up => {
                if (self.prev_index / size) - (self.current_index / size) > 5 {
                    self.current_index + size
                } else {
                    self.current_index - 1
                }
            },

            Direction::UpRight => {
                if self.off == 0b1001 {
                    self.current_index - 1
                } else {
                    self.current_index - size + 1
                }
            },

            Direction::Left => {
                if prev_area.timing == TOP || self.current_index % size == self.prev_index % size && self.current_index != self.prev_index {
                    return self.current_index + size
                }

                if self.free & 1 == 1 {
                    self.current_index - size + 1
                } else if self.free == 0b1110 {
                    self.current_index + size + 1
                } else {
                    self.current_index - 1
                }
            },
            _ => self.current_index - 1
        }
    }

    fn near_timing(&self, size: usize, prev_area: Area) -> usize {
        match self.guess_direction(prev_area, size) {
            Direction::DownRight => {
                if self.off == 0b0110 {
                    self.current_index - 1
                } else {
                    self.current_index + size + 1
                }
            },

            Direction::JumpLeft => {
                
            },

            Direction::UpRight => {
                if self.same_row() && prev_area.same_row() {
                    self.current_index % size - 1
                } else if self.free & 0b0011 == 0 {
                    self.current_index - 1
                } else if self.off == 0b1001 {
                    self.current_index - 1                    
                } else {
                    self.current_index - size + 1
                }
            },

            Direction::Left => {
                if self.timing == BOTTOM && prev_area.free == UPPER_LEFT {
                    self.current_index - size + 1
                } else if self.timing == TOP {
                    self.current_index - 1
                } else {
                    self.current_index - 1
                }
                // if self.timing == BOTTOM {

                // } else if {

                // }
                // if prev_area.timing == TOP && self.timing == TOP {
                //     if prev_area.prev_index == self.current_index + size {
                //         self.current_index - (size * 2) + 1
                //     } else {
                //         self.current_index % size - 1
                //     }
                // } else if self.timing == BOTTOM {
                //     if self.msg == TOP {
                //         // implies a downward motion
                //         self.current_index - 1
                //     } else if self.free == UPPER_LEFT {
                //         // implies an upward motion if it's not downward?
                //     } else {
                //         self.current_index + (size * 2)
                //     }
                // } else {
                //     self.current_index - 1
                // }
                // // if self.free & 1 == 1 {
                // //     self.current_index - size + 1
                // // } else if self.free == 0b1110 {
                // //     self.current_index + size + 1
                // // } else {
                // //     self.current_index - 1
                // // }
            },

            Direction::UpLeft => {
                self.current_index % size - 1
            },

            _ => self.current_index - 1
        }
    }

    pub fn adjust_position(&self, size: usize, prev_area: Area) -> usize {
        self.print_binary(size, prev_area);

        if self.timing > 0 {
            self.near_timing(size, prev_area)
        } else if self.off > 0 {
            self.near_edge(size, prev_area)
        } else if self.algn > 0 {
            self.near_alignment(size, prev_area)
        } else if self.msg == 0b0010 {
            self.current_index - size + 1
        } else if self.msg == 0b0001 {
            self.current_index + size + 1
        } else {
            self.current_index - 1
        }
    }
}