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


// message length payloads vary, it looks like and may take up to 16 bits. Assume 8 bits for now.
pub fn is_msg_len(row: usize, col: usize, size: usize) -> bool {
    row >= size - 7 && row < size - 3 && col >= size - 3
}

fn encode_chunk(index: &mut usize, byte: u8, bit_count: isize, bits: &[Bit]) {
    while i >= 0 {
        let ref mut bit = bits[index];
        bit.filled = true;
        bit.val = (byte & (1 << shift)) == 0;
    }
}

pub fn encode(message: String, mode: u8, size: usize, qr: QRGrid) {
    let msg_length = message.len();
    let mut index = (size * size) - 1;
    let mut chunked_bits = qr.chunk_bits(index);
    encode_chunk(index, mode, 3, chunked_bits);

    for byte in message.into_bytes() {
        encode_chunk(index, byte, 7, &qr.bits);
    }
}
