pub fn is_fixed_area(x: usize, y: usize, size: usize) -> bool {
    x <= 7 && (y <= 7 || (size - y) <= 7) || y <= 7 && (size - x) <= 7
}

pub fn is_bridge_area(x: usize, y: usize, size: usize) -> bool {
    x == 6 && (y >= 8 && y <= size - 9) || x >= 8 && x <= (size - 9) && y == 6
}

pub fn is_format_area(x: usize, y: usize, size: usize) -> bool {
    if x == 8 {
        match y {
            0...8 | 42...48 => true,
            _ => false
        }
    } else if y == 8 {
        match x {
            0...8 | 42...48 => true,
            _ => false
        }
    } else {
        false
    }
}


// the encoding type might vary in length but for now, assume 4 bits
pub fn is_enc_type(row: usize, col: usize, size: usize) -> bool {
    row >= size - 3 && col >= size - 3
}

pub fn available_paths(row: usize, col: usize, size: usize) -> usize {
    if row == 0 || col == 0 {
        return 2usize
    }
    let avail_paths: [(usize, usize); 4] = [
        (row - 1, col),
        (row + 1, col),
        (row, col - 1),
        (row, col + 1)
    ];
    let mut count = 4;
    for path in avail_paths.into_iter() {
        let (x, y) = *path;
        if is_format_area(x, y, size) || is_bridge_area(x, y, size) || is_fixed_area(x, y, size) {
            count -= 1;
        }
    }
    count
}

// message length payloads vary, it looks like and may take up to 16 bits. Assume 8 bits for now.
pub fn is_msg_len(row: usize, col: usize, size: usize) -> bool {
    row >= size - 7 && row < size - 3 && col >= size - 3
}
