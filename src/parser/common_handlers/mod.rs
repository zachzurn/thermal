pub mod graphics_data;


pub fn as_pbm(width: u32, height: u32, bytes: &Vec<u8>) -> Vec<u8> {
    let dim = format!("{} {}", width, height);
    let dimbytes = dim.as_bytes();

    let mut header: Vec<u8> = vec![0x50, 0x34, 0x0A];

    for b in dimbytes { header.push(*b) }

    header.extend(bytes.to_owned());
    header
}