pub fn bits(byte: u8) -> (bool,bool,bool,bool,bool,bool,bool,bool) {
    ((byte>>0) % 2 != 0, (byte>>1) % 2 != 0, (byte>>2) % 2 != 0, (byte>>3) % 2 != 0, (byte>>4) % 2 != 0, (byte>>5) % 2 != 0, (byte>>6) % 2 != 0, (byte>>7) % 2 != 0)
}
