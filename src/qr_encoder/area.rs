pub struct Area {
    pub timing: u8,
    pub msg: u8,
    pub algn: u8,
    pub off: u8,
    pub free: u8
}

#[derive(Debug)]
enum Direction {
    Left,
    Up,
    UpRight,    
    UpLeft,
    Down,
    DownLeft,
    DownRight
}

impl Area {
    fn get_direction(&self, prev_index: usize, current_index: usize) -> Direction {
        let same_row: bool = current_index + 1 == prev_index;

        if same_row && self.msg & 0b0001 == 1 || self.free == 0b0110 && self.timing == 0 {
            Direction::DownRight
        } else if same_row && self.msg & 0b0010 == 2 || self.free == 0b1001 || self.free == 0b1101 {
            Direction::UpRight
        } else if !same_row && prev_index > current_index && self.free & 0b1100 > 0 {
            Direction::Up
        } else if !same_row && prev_index < current_index && self.free == 0b0010 {
            Direction::Down
        } else if same_row && self.timing == 0b1001 {
            Direction::UpLeft
        } else {
            Direction::Left
        }
    }

    pub fn near_alignment(&self, prev_index: usize, current_index: usize, size: usize) -> usize {
        match self.get_direction(prev_index, current_index) {
            Direction::UpRight => {
                if self.algn == 0b1001 {
                    current_index - (size * 6) + 1
                } else if self.algn & 1 == 1 {
                    current_index - size
                } else if self.algn == 0b0110 || self.free == 0b1101 {
                    current_index - size + 1
                } else {
                    current_index - 1
                }
            },

            Direction::DownRight => {
                if self.free == 0b0110 {
                    current_index + size + 1
                } else if self.algn == 0b0110 {
                    current_index + (size * 6) + 1
                } else if self.algn == 0b1100 {
                    current_index + size + 1
                } else if self.algn & 0b0010 == 0b0010 {
                    current_index + size
                } else if self.algn & 0b0100 == 0b0100 {
                    current_index + size + 1
                } else {
                    current_index - 1
                }
            },

            Direction::Up => {
                if self.free == 0b1100 {
                    current_index - size
                } else if self.free == 0b1101 {
                    current_index - size + 1
                } else {
                    current_index - 1
                }
            },

            Direction::Left => {
                if self.algn == 0b1100 {
                    current_index + size + 1
                } else if self.free == 0b1100 {
                    current_index - size
                } else {
                    current_index - 1
                }
            },
            _ => current_index - 1
        }
    }

    pub fn print_binary(&self, prev_index: usize, current_index: usize, size: usize) {
        println!("Direction: {:?}", self.get_direction(prev_index, current_index));
        println!("Point: {}, {}", current_index / size, current_index % size);
        println!("Alignment {:b}", self.algn);
        println!("Offsides  {:b}", self.off);
        println!("Free      {:b}", self.free);
        println!("Message   {:b}", self.msg);
        println!("Timing    {:b}", self.timing);
    }

    pub fn near_edge(&self, prev_index: usize, current_index: usize, size: usize) -> usize {
        match self.get_direction(prev_index, current_index) {
            Direction::DownRight => {
                if self.off == 0b0110 {
                    current_index - 1
                } else {
                    current_index + size + 1
                }
            },
            Direction::UpRight => {
                if self.off == 0b1001 {
                    current_index - 1
                } else {
                    current_index - size + 1
                }
            },
            Direction::Left => {
                if self.free & 1 == 1 {
                    current_index - size + 1
                } else if self.free == 0b1110 {
                    current_index + size + 1
                } else {
                    current_index - 1
                }
            },
            _ => current_index - 1
        }
    }

    fn near_timing(&self, prev_index: usize, current_index: usize, size: usize) -> usize {
        match self.get_direction(prev_index, current_index) {
            Direction::DownRight => {
                if self.off == 0b0110 {
                    current_index - 1
                } else {
                    current_index + size + 1
                }
            },

            Direction::UpRight => {
                if self.free & 0b0011 == 0 {
                    current_index - 1
                } else if self.off == 0b1001 {
                    current_index - 1                    
                } else {
                    current_index - size + 1
                }
            },

            Direction::Left => {
                if self.free & 1 == 1 {
                    current_index - size + 1
                } else if self.free == 0b1110 {
                    current_index + size + 1
                } else {
                    current_index - 1
                }
            },

            Direction::UpLeft => {
                current_index % size - 1
            },

            _ => current_index - 1
        }
    }

    pub fn adjust_position(&self, prev_index: usize, current_index: usize, size: usize) -> usize {
        self.print_binary(prev_index, current_index, size);
        if self.timing > 0 {
            self.near_timing(prev_index, current_index, size)
        } else if self.off > 0 {
            self.near_edge(prev_index, current_index, size)
        } else if self.algn > 0 {
            self.near_alignment(prev_index, current_index, size)
        } else if self.msg == 0b0010 {
            current_index - size + 1
        } else if self.msg == 0b0001 {
            current_index + size + 1
        } else {
            current_index - 1
        }
    }
}