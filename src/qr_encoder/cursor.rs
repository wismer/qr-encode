use qr_encoder::cell::{Point};
use std::fmt;


// cardinal points

const N: u8 = 1;
const NE: u8 = N << 1;
const E: u8 = N << 2;
const SE: u8 = N << 3;
const S: u8 = N << 4;
const SW: u8 = N << 5;
const W: u8 = N << 6;
const NW: u8 = N << 7;

const NNE: u8 = N | NE;
const ENE: u8 = E | NE;
const ESE: u8 = E | SE;
const SSE: u8 = SE | S;
const SSW: u8 = S | SW;
const WSW: u8 = SW | W;
const WNW: u8 = W | NW;
const NNW: u8 = NW | N;


const TOP: u8 = 131;         // 0b10000011
const BOTTOM: u8 = 56;       // 0b00111000
const LEFT: u8 = 224;        // 0b11100000
const RIGHT: u8 = 14;        // 0b00001110;

const UR_CORNER: u8 = TOP | RIGHT;
const LL_CORNER: u8 = LEFT | BOTTOM;
const UL_CORNER: u8 = LEFT | TOP;

const LR_LEDGE: u8 = RIGHT << 1;

pub struct QRContext {
    pub free: u8,
    pub algn: u8,
    pub timing: u8,
    pub off: u8,
    pub msg: u8
}

pub struct Cursor {
    pub drawn_path: Vec<usize>,
    pub current_index: usize,
    pub context: QRContext
}

fn count_bits(n: u8) -> usize {
    let mut count = 0;
    for i in 0..8 {
        let bit = (n >> i) & 1;
        if bit == 1 {
            count += 1;
        }
    }

    count
}

impl fmt::Display for QRContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "Position: CurrentIndex = {}, previous_index = {}", self.current_index, self.prev_index)
        let mut start: u8 = 0b10000000;
        let mut context_canvas = String::from("");
        let order = [7, 0, 1, 6, 9, 2, 5, 4, 3];

        /*
            top = 7 0 1
            middle 6 X 2
            bottom 5 4 3
        */

        for i in order.iter() {
            if *i == 9 {
                context_canvas.push_str(" Q ");
                continue;
            }

            let pos = (1 << i) as u8;

            if self.free & pos == pos {
                context_canvas.push_str(" F ");
            } else if self.algn & pos == pos {
                context_canvas.push_str(" A ");
            } else if self.off & pos == pos {
                context_canvas.push_str(" X ");
            } else if self.timing & pos == pos {
                context_canvas.push_str(" T ");
            } else if self.msg & pos == pos {
                context_canvas.push_str(" M ");
            } else {
                context_canvas.push_str(" E ");
            }

            if *i == 1 {
                context_canvas.push_str("\n");
            } else if *i == 2 {
                context_canvas.push_str("\n");
            }
        }

        write!(f, "{}", context_canvas)
    }
}


impl Cursor {
    fn timing(&self, canvas_size: isize, idx: isize) -> isize {
        let msg = self.context.msg;
        let timing = self.context.timing;
        let free = self.context.free;
        let col = idx % canvas_size;
        let row = idx / canvas_size;

        if canvas_size > 43 && row == 7 && col == canvas_size - 10 {
            return -(canvas_size * 7) - 2
        }

        match timing {
            TOP if msg == (S | SE | E) => -(canvas_size * 2) + 1,
            BOTTOM if msg == (N | NE | E) => canvas_size * 2 + 1,
            _ => -1
        }
    }

    fn offside(&self, canvas_size: isize, idx: isize) -> isize {
        let off = self.context.off;
        let free = self.context.free;

        let col = idx % canvas_size;
        let row = idx / canvas_size;

        if canvas_size > 43 && col == canvas_size - 12 && (off == RIGHT || off == UR_CORNER) {
            return canvas_size
        }

        if row == canvas_size - 1 && free == 0 {
            -(canvas_size * 8) - 1
        } else if row == 0 && free == 0 {
            (canvas_size * 8) - 1
        } else {
            -1
        }
    }

    fn timing_alignment(&self, canvas_size: isize, idx: isize) -> isize  {
        let timing = self.context.timing;
        let algn = self.context.algn;
        let free = self.context.free;
        let prev_index = self.drawn_path[1] as isize;

        if prev_index == idx - (canvas_size * 2) {
            return canvas_size
        }

        match algn {
            TOP if timing & LEFT > 0 => -(canvas_size * 6) + 1,
            BOTTOM if timing & LEFT > 0 => (canvas_size * 6) + 1,

            RIGHT if timing & BOTTOM > 0 && free == (W | NW) => canvas_size * 2,
            RIGHT if timing & TOP > 0 && free == (W | SW) => -canvas_size * 2,
            RIGHT if timing & TOP > 0 && free == (W | NW) => -canvas_size * 2,
            RIGHT if timing & BOTTOM > 0 && free == 0 => (canvas_size * 2) + 1,
            RIGHT if timing & BOTTOM > 0 && prev_index == (idx + (canvas_size * 2)) => -canvas_size,
            LEFT if timing & TOP > 0 && free == 0 => -(canvas_size * 2) + 1,
            LEFT if timing & BOTTOM > 0 && free == 0 => (canvas_size * 2) + 1,
            _ => -1
        }
    }

    fn alignment(&self, canvas_size: isize, idx: isize) -> isize {
        let algn = self.context.algn;
        let msg = self.context.msg;
        let free = self.context.free;
        let prev_index = self.drawn_path[1] as isize;


        match algn {
            NE if prev_index == idx + canvas_size - 1 => -1,
            ENE if msg & SW == SW => -1,
            NE | SW => -canvas_size,
            ESE if prev_index == idx + canvas_size - 1 => -1,
            SE if prev_index == idx + canvas_size - 1 => -1,
            ESE | SE | NW => canvas_size,
            ENE => -canvas_size,
            WNW => -canvas_size,
            SSE if free == LEFT => (canvas_size * 6) + 1,
            NNW if msg & SW == SW => -1,
            TOP if free == W => -1,
            TOP if msg & S == S => -(canvas_size * 6) + 1,
            BOTTOM if msg == (N | NE | E) => (canvas_size * 6) + 1,
            RIGHT if prev_index == idx + canvas_size - 1 => -1,
            NNE | RIGHT | LEFT if free & N == N => -canvas_size,
            RIGHT | LEFT => canvas_size,
            _ => -1
        }
    }

    fn timing_offside(&self, canvas_size: isize, idx: isize) -> isize {
        let off = self.context.off;
        let timing = self.context.timing;
        let row = idx / canvas_size;
        let free = self.context.free;
        if canvas_size > 43 && row == 5 {
            return (canvas_size * 2) + 1
        }

        if off == TOP && timing & LEFT > 0 {
            return -2
        }

        if off == LEFT && timing & BOTTOM > 0 && free == 0 {
            return (canvas_size * 2) + 1
        }

        -1 // I think?
    }

    fn get_next_position(&self, canvas_size: isize) -> usize {
        let current_index = self.current_index as isize;
        println!("{} \n with index being {:?} \n {:?}", self.context, (current_index / canvas_size, current_index % (canvas_size)), self.drawn_path);

        let modifier: isize = if self.context.free & SE == SE {
            canvas_size + 1
        } else if self.context.free & NE == NE {
            -canvas_size + 1
        } else if self.context.timing > 0 && self.context.algn > 0 {
            self.timing_alignment(canvas_size, current_index)
        } else if self.context.algn > 0 {
            self.alignment(canvas_size, current_index)
        } else if self.context.timing > 0 && self.context.off > 0 {
            self.timing_offside(canvas_size, current_index)
        } else if self.context.off > 0 {
            self.offside(canvas_size, current_index)
        } else if self.context.timing > 0 {
            self.timing(canvas_size, current_index)
        } else {
            -1
        };

        (current_index + modifier) as usize
    }

    pub fn reset_context(&mut self) {
        self.context = QRContext {
            free: 0,
            msg: 0,
            timing: 0,
            algn: 0,
            off: 0
        };
    }

    pub fn move_cursor(&mut self, canvas_size: usize) {
        if self.drawn_path.len() == 8 {
            self.drawn_path.pop();
        }

        self.drawn_path.insert(0, self.current_index);
        self.current_index = self.get_next_position(canvas_size as isize);
        self.reset_context();
    }
}
