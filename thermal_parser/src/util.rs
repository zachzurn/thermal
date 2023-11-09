pub fn bitflags_lsb(byte: u8) -> [bool; 8] {
    let test = 2;
    [
        (byte >> 0) % test != 0,
        (byte >> 1) % test != 0,
        (byte >> 2) % test != 0,
        (byte >> 3) % test != 0,
        (byte >> 4) % test != 0,
        (byte >> 5) % test != 0,
        (byte >> 6) % test != 0,
        (byte >> 7) % test != 0,
    ]
}

pub fn bitflags_msb(byte: u8) -> [bool; 8] {
    let test = 2;
    [
        (byte >> 7) % test != 0,
        (byte >> 6) % test != 0,
        (byte >> 5) % test != 0,
        (byte >> 4) % test != 0,
        (byte >> 3) % test != 0,
        (byte >> 2) % test != 0,
        (byte >> 1) % test != 0,
        (byte >> 0) % test != 0,
    ]
}
