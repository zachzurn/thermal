use crate::context::HumanReadableInterface;
use crate::text::TextSpan;
use crate::util::parse_u16;

#[derive(Clone)]
pub struct Barcode {
    pub points: Vec<u8>,
    pub point_width: u8,
    pub point_height: u8,
    pub hri: HumanReadableInterface,
    pub text: TextSpan,
}

#[derive(Clone, Debug)]
pub enum VectorGraphic {
    Rectangle(Rectangle),
}

#[derive(Clone, Debug)]
pub struct Rectangle {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

#[derive(Clone)]
pub struct Line {
    _ax: u32,
    _ay: u32,
    _bx: u32,
    _by: u32,
}

#[derive(Clone)]
pub struct Code2D {
    pub points: Vec<u8>,
    pub width: u32,
    pub point_width: u32,
    pub point_height: u32,
}

#[derive(Clone, Debug)]
pub enum ImageFlow {
    Inline, //Image acts somewhat like text, advances x until line is full
    Block,  //Image advances y by height and resets x to 0
    None,   //Image does not advance xy
}

#[derive(Clone, Debug)]
pub struct Image {
    pub pixels: Vec<u8>,
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
    pub stretch: (u8, u8),
    pub flow: ImageFlow,
    pub upside_down: bool,
}

#[derive(Clone, Debug)]
pub struct RenderColors {
    pub(crate) paper_color: (u8, u8, u8),
    pub(crate) color_1: (u8, u8, u8),
    pub(crate) color_2: (u8, u8, u8),
    pub(crate) color_3: (u8, u8, u8),
    pub(crate) debug_color_1: (u8, u8, u8),
    pub(crate) debug_color_2: (u8, u8, u8),
    pub(crate) debug_color_3: (u8, u8, u8),
}

impl Image {
    //TODO get rid of this
    pub fn as_grayscale(&self) -> Vec<u8> {
        return self.pixels.clone();
    }

    pub fn unpack_bit_encoding(&mut self) {
        let mut bytes = Vec::<u8>::new();

        //number of bytes we need to use for the last column of each row of data
        let mut padding = self.w % 8;
        if padding == 0 {
            padding = 8;
        }
        let mut col = 0;

        for byte in &self.pixels {
            col += 8;
            if col >= self.w {
                for n in 0..padding {
                    bytes.push(if *byte & 1 << (7 - n) != 0 { 0 } else { 255 });
                }
                col = 0;
            } else {
                bytes.push(if *byte & 1 << 7 != 0 { 0 } else { 255 });
                bytes.push(if *byte & 1 << 6 != 0 { 0 } else { 255 });
                bytes.push(if *byte & 1 << 5 != 0 { 0 } else { 255 });
                bytes.push(if *byte & 1 << 4 != 0 { 0 } else { 255 });
                bytes.push(if *byte & 1 << 3 != 0 { 0 } else { 255 });
                bytes.push(if *byte & 1 << 2 != 0 { 0 } else { 255 });
                bytes.push(if *byte & 1 << 1 != 0 { 0 } else { 255 });
                bytes.push(if *byte & 1 << 0 != 0 { 0 } else { 255 });
            }
        }

        self.pixels = bytes
    }

    pub fn from_raster_data(data: &Vec<u8>) -> Option<Image> {
        if data.len() < 8 {
            return None;
        };

        let a = *data.get(0).unwrap();
        let bx = *data.get(1).unwrap();
        let by = *data.get(2).unwrap();
        let c = *data.get(3).unwrap();

        let mut width = parse_u16(data, 4) as u32;
        let mut height = parse_u16(data, 6) as u32;

        let stretch = (bx, by);

        let pixels = if bx > 1 || by > 1 {
            let (w, h, mut px) = scale_pixels(&data[8..], width as u32, height as u32, bx, by);
            width = w;
            height = h;

            pack_color_levels(&mut px, 1);

            px
        } else {
            let mut pixels = data[8..].to_vec();
            pack_color_levels(&mut pixels, c);
            pixels
        };

        let mut img = Image {
            pixels,
            x: 0,
            y: 0,
            w: width as u32,
            h: height as u32,
            stretch,
            flow: ImageFlow::Block,
            upside_down: false,
        };

        img.unpack_bit_encoding();

        Some(img)
    }

    pub fn from_raster_data_with_ref(
        data: &Vec<u8>,
        storage: ImageRefStorage,
    ) -> Option<(ImageRef, Image)> {
        if data.len() < 8 {
            return None;
        };

        let a = *data.get(0).unwrap();
        let kc1 = *data.get(1).unwrap();
        let kc2 = *data.get(2).unwrap();
        let b = *data.get(3).unwrap(); //Number of colors
        let c = *data.get(8).unwrap();

        let width = parse_u16(data, 4) as u32;
        let height = parse_u16(data, 6) as u32;

        //b (above) specifies number of color data stored,
        // we are ignoring this for now if b > 1
        // [byte(color) bytes(capacity)] [byte(color) bytes(capacity)]
        //TODO implement with a test

        let stretch = (1, 1);

        let mut pixels = data[9..].to_vec();

        pack_color_levels(&mut pixels, c);

        let mut img = Image {
            pixels,
            x: 0,
            y: 0,
            w: width,
            h: height,
            stretch,
            flow: ImageFlow::None,
            upside_down: false,
        };

        img.unpack_bit_encoding();

        Some((ImageRef { kc1, kc2, storage }, img))
    }

    pub fn from_column_data(data: &Vec<u8>) -> Option<Image> {
        if data.len() < 8 {
            return None;
        };

        let a = *data.get(0).unwrap();
        let bx = *data.get(1).unwrap();
        let by = *data.get(2).unwrap();

        let c = *data.get(3).unwrap();

        let mut width = parse_u16(data, 4) as u32;
        let mut height = parse_u16(data, 6) as u32;

        let stretch = (bx, by);

        let pixels = if bx > 1 || by > 1 {
            let (w, h, mut px) = scale_pixels(&data[8..], width as u32, height as u32, bx, by);
            width = w;
            height = h;

            pack_color_levels(&mut px, c);

            px
        } else {
            let mut pixels = data[8..].to_vec();
            pack_color_levels(&mut pixels, c);
            pixels
        };

        let mut img = Image {
            pixels,
            x: 0,
            y: 0,
            w: width,
            h: height,
            stretch,
            flow: ImageFlow::None,
            upside_down: false,
        };

        img.unpack_bit_encoding();

        Some(img)
    }

    pub fn from_column_data_with_ref(
        data: &Vec<u8>,
        storage: ImageRefStorage,
    ) -> Option<(ImageRef, Image)> {
        if data.len() < 8 {
            return None;
        };

        let a = *data.get(0).unwrap();
        let kc1 = *data.get(1).unwrap();
        let kc2 = *data.get(2).unwrap();
        let b = *data.get(3).unwrap(); //Number of color data

        let width = parse_u16(data, 4) as u32;
        let height = parse_u16(data, 6) as u32;

        let stretch = (1, 1);

        let mut pixels = data[8..].to_vec();

        pack_color_levels(&mut pixels, 1);

        let mut img = Image {
            pixels,
            x: 0,
            y: 0,
            w: width,
            h: height,
            stretch,
            flow: ImageFlow::None,
            upside_down: false,
        };

        img.unpack_bit_encoding();

        Some((ImageRef { kc1, kc2, storage }, img))
    }
}

/// Converts column data, which is encoded in
/// 1 bit per pixel (LSB) into 1 byte per pixel.
/// column data also needs to be rotated and
/// flipped in order to print correctly.
///
/// Ideally, the operations can be done directly
/// on the bits. If you are reading this and can
/// contribute a function for doing this, we will
/// pull it into the repo.
pub fn column_to_raster(
    pixels: &[u8],
    stretch: (u8, u8),
    final_width: u32,
    final_height: u32,
) -> (u32, u32, Vec<u8>) {
    let width = final_height;
    let mut bytes = Vec::<u8>::new();

    //number of bytes we need to use for the last column of each row of data
    let mut padding = width % 8;
    if padding == 0 {
        padding = 8;
    }
    let mut col = 0;

    for byte in pixels {
        col += 8;
        if col >= width {
            for n in 0..padding {
                bytes.push(if *byte & 1 << (7 - n) != 0 { 0 } else { 255 });
            }
            col = 0;
        } else {
            bytes.push(if *byte & 1 << 7 != 0 { 0 } else { 255 });
            bytes.push(if *byte & 1 << 6 != 0 { 0 } else { 255 });
            bytes.push(if *byte & 1 << 5 != 0 { 0 } else { 255 });
            bytes.push(if *byte & 1 << 4 != 0 { 0 } else { 255 });
            bytes.push(if *byte & 1 << 3 != 0 { 0 } else { 255 });
            bytes.push(if *byte & 1 << 2 != 0 { 0 } else { 255 });
            bytes.push(if *byte & 1 << 1 != 0 { 0 } else { 255 });
            bytes.push(if *byte & 1 << 0 != 0 { 0 } else { 255 });
        }
    }

    pack_color_levels(&mut bytes, 1);
    let rot = rotate_90_clockwise(bytes, final_height, final_width);
    let mut flip = flip_right_to_left(rot, final_width, final_height);

    if stretch.0 > 1 || stretch.1 > 1 {
        scale_pixels(&flip, final_width, final_height, stretch.0, stretch.1)
    } else {
        (final_width, final_height, flip)
    }
}

fn rotate_90_clockwise(data: Vec<u8>, width: u32, height: u32) -> Vec<u8> {
    let mut result = vec![0; data.len()];

    for y in 0..height {
        for x in 0..width {
            let src_index = y as usize * width as usize + x as usize;
            let dest_x = height - 1 - y;
            let dest_y = x;
            let dest_index = dest_y as usize * height as usize + dest_x as usize; // Note: height is used for new row length
            result[dest_index] = data[src_index];
        }
    }

    result
}

fn flip_right_to_left(data: Vec<u8>, width: u32, height: u32) -> Vec<u8> {
    let mut result = vec![0; data.len()];

    for y in 0..height {
        for x in 0..width {
            let src_index = y as usize * width as usize + x as usize;
            let dest_x = width - 1 - x; // Flip the x-coordinate
            let dest_index = y as usize * width as usize + dest_x as usize; // Calculate the destination index
            result[dest_index] = data[src_index];
        }
    }

    result
}

// Colors are stored in greyscale (84 levels)
// 0 is a debug color
// 1 - 84 is color 1
// 85 - 168 is color 2
// 170 - 252 is color 3
// 0, 253 and 254 are used for debug colors
pub fn pack_color_levels(pixels: &mut Vec<u8>, color: u8) {
    // Calculate the starting value based on the color
    let pack_start = match color {
        // 0  253 254 and 255 are reserved
        1 | 49 => 1,    // Range: 1-84
        2 | 50 => 85,   // Range: 85-168
        3 | 51 => 169,  // Range: 169-252
        _ => 1,
    };

    for p in pixels.iter_mut() {
        *p = if *p > 252 {
            pack_start + 83 // Map 255 to the max of the range (84, 168, or 252)
        } else {
            pack_start + (*p as u16 * 83 / 255) as u8
        };
    }
}

/// Converts packed bytes into full ru8 gu8 bu8 bytes.
/// This whole library uses packed bytes for color data.
///
/// All Image graphics are encoded with the packed byte format.
///
/// Use this function to convert back to rgb pixels for
/// encoding png and other image formats.
pub fn unpack_color_levels(
    pixels: &mut Vec<u8>,
    render_colors: &RenderColors
) -> Vec<u8> {
    let mut unpacked_pixels = Vec::with_capacity(pixels.len() * 3);

    for &p in pixels.iter() {
        let (r, g, b) = {
            let (target_color, level) = match p {
                0 => (render_colors.debug_color_1, 0),
                1..=84 => (render_colors.color_1, p - 1), // 0 - 84
                85..=168 => (render_colors.color_2, p - 84),
                169..=252 => (render_colors.color_3, p - 168),
                253 => (render_colors.debug_color_2, 0),
                254 => (render_colors.debug_color_3, 0),
                _ => (render_colors.paper_color, 0), //255 is paper color
            };

            // How much is the key color used 0 = 100%
            // 84 = 0 which is fully transparent 
            // 0 = 1 which is fully visible
            let opacity = 1.0 - (level as f32 / 84.0);

            (
                blend_channel(render_colors.paper_color.0, target_color.0, opacity),
                blend_channel(render_colors.paper_color.1, target_color.1, opacity),
                blend_channel(render_colors.paper_color.2, target_color.2, opacity),
            )
        };

        unpacked_pixels.extend_from_slice(&[r, g, b]);
    }

    unpacked_pixels
}

// opacity 0 = full base
// opacity 1 = full target
// paper = 255
// color = 0
// opacity = 1
// blended should = 0
fn blend_channel(base: u8, target: u8, opacity: f32) -> u8 {
    let blended = (target as f32 * opacity) + (base as f32 * (1.0 - opacity));
    blended.round() as u8
}

pub fn scale_pixels(
    bytes: &[u8],
    original_width: u32,
    original_height: u32,
    scale_x: u8,
    scale_y: u8,
) -> (u32, u32, Vec<u8>) {
    let scale_x = scale_x.max(1);
    let scale_y = scale_y.max(1);

    let new_width = original_width * scale_x as u32;
    let new_height = original_height * scale_y as u32;

    let mut scaled_bytes = vec![0u8; (new_width * new_height) as usize];

    for y in 0..original_height {
        for x in 0..original_width {
            let pixel = bytes[(y * original_width + x) as usize];

            for dy in 0..scale_y {
                for dx in 0..scale_x {
                    let new_x = x * scale_x as u32 + dx as u32;
                    let new_y = y * scale_y as u32 + dy as u32;
                    scaled_bytes[(new_y * new_width + new_x) as usize] = pixel;
                }
            }
        }
    }

    (new_width, new_height, scaled_bytes)
}

//Images that were added to storage can be
//referenced with an ImageRef
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ImageRef {
    pub kc1: u8,
    pub kc2: u8,
    pub storage: ImageRefStorage,
}

impl ImageRef {
    pub fn from_data(data: &Vec<u8>, storage: ImageRefStorage) -> Option<ImageRef> {
        if data.len() < 2 {
            return None;
        }
        Some(ImageRef {
            kc1: *data.get(0).unwrap(),
            kc2: *data.get(1).unwrap(),
            storage,
        })
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum ImageRefStorage {
    Disc,
    Ram,
}

pub enum GraphicsCommand {
    Error(String),
    Code2D(Code2D),
    Barcode(Barcode),
    Image(Image),
    Rectangle(Rectangle),
    Line(Line),
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pack_color_levels() {
        let mut sample: Vec<u8> = vec![0,122,255];
        pack_color_levels(&mut sample, 1);

        let mut sample_2: Vec<u8> = vec![0,122,255];
        pack_color_levels(&mut sample_2, 2);

        let mut sample_3: Vec<u8> = vec![0,122,255];
        pack_color_levels(&mut sample_3, 3);

        println!("Packed color 1 {:?}", sample);
        println!("Packed color 2 {:?}", sample_2);
        println!("Packed color 3 {:?}", sample_3);
    }
}