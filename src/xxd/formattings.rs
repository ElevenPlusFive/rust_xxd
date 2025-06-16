
pub fn get_hex_section(input: &[u8]) -> [u8; 40] {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out: [u8; 40] = [0; 40];

    // Zero the entire buffer first
    out.fill(b' ');

    for (i, pair) in input.chunks(2).enumerate() {
        let offset = i * 5;

        let first_high_nibble = (pair[0] >> 4) as usize;
        let first_low_nibble = (pair[0] & 0x0F) as usize;
        out[offset] = HEX[first_high_nibble];
        out[offset + 1] = HEX[first_low_nibble];

        if pair.len() < 2 {
            break;
        }

        let second_high_nibble = (pair[1] >> 4) as usize;
        let second_low_nibble = (pair[1] & 0x0F) as usize;

        out[offset + 2] = HEX[second_high_nibble];
        out[offset + 3] = HEX[second_low_nibble];
        out[offset + 4] = b' ';
    }

    out
}

pub fn get_ascii_section(arr: &[u8]) -> [u8; 16] {
    const ASCII_PRINTABLE_LOWER_BOUND: u8 = 0x20;
    const ASCII_PRINTABLE_UPPER_BOUND: u8 = 0x7e;

    let mut output: [u8; 16] = [0; 16];

    output.fill(b' ');

    for (i, &x) in arr.iter().enumerate().take(16) {
        output[i] = match x {
            ASCII_PRINTABLE_LOWER_BOUND..=ASCII_PRINTABLE_UPPER_BOUND => x,
            _ => b'.',
        };
    }

    output
}
