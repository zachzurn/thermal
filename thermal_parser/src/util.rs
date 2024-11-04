pub fn parse_u16(bytes: &Vec<u8>, index: usize) -> u16 {
    let low = bytes.get(index).unwrap_or(&0);
    let high = bytes.get(index + 1).unwrap_or(&0);

    ((*high as u16) << 8) | *low as u16
}

type BitFlags = (bool, bool, bool, bool, bool, bool, bool, bool);

pub fn bitflags_lsb(byte: &u8) -> BitFlags {
    (
        (byte & (1 << 0)) != 0,
        (byte & (1 << 1)) != 0,
        (byte & (1 << 2)) != 0,
        (byte & (1 << 3)) != 0,
        (byte & (1 << 4)) != 0,
        (byte & (1 << 5)) != 0,
        (byte & (1 << 6)) != 0,
        (byte & (1 << 7)) != 0,
    )
}

pub fn bitflags_msb(byte: &u8) -> BitFlags {
    (
        (byte & (1 << 7)) != 0,
        (byte & (1 << 6)) != 0,
        (byte & (1 << 5)) != 0,
        (byte & (1 << 4)) != 0,
        (byte & (1 << 3)) != 0,
        (byte & (1 << 2)) != 0,
        (byte & (1 << 1)) != 0,
        (byte & (1 << 0)) != 0,
    )
}
