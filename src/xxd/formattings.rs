pub fn get_line_index(index: usize) -> String {
    let display_index = index*16;
    format!("{display_index:08x}")
}

pub fn get_pairs_section(arr: &[u8]) -> String {
    arr.chunks(2)
        .map(|chunk|  match chunk {
            [first, second] => format!("{first:02x}{second:02x}"),
            [first] => format!("{first:02x}"),
            _ => "".to_string()
        })
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn get_ascii_section(arr: &[u8]) -> String {
    const ASCII_PRINTABLE_LOWER_BOUND: u8 = 0x20;
    const ASCII_PRINTABLE_UPPER_BOUND: u8 = 0x7e;

    arr.iter()
        .map(|x| match x {
            ASCII_PRINTABLE_LOWER_BOUND..=ASCII_PRINTABLE_UPPER_BOUND => char::from(*x),
            _ => '.'
        })
        .collect::<String>()
}
