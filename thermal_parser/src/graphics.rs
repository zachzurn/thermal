use crate::context::HumanReadableInterface;

#[derive(Clone)]
pub struct Barcode {
    pub points: Vec<u8>,
    pub point_width: u8,
    pub point_height: u8,
    pub hri: HumanReadableInterface,
    pub text: String,
}

#[derive(Clone)]
pub struct Rectangle {
    _x: u32,
    _y: u32,
    _width: u32,
    _height: u32,
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

#[derive(Clone, PartialEq)]
pub enum PixelType {
    //1 byte per pixel
    MonochromeByte,
    //1 bit per pixel one color, the u8 selects the color (1 - 4)
    Monochrome(u8),
    //the first u8 selects the color (1 - 4), second how many colors are in the data
    MultipleTone(u8, u8),
    Unknown,
}

#[derive(Clone)]
pub struct Image {
    pub pixels: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub pixel_type: PixelType,
    pub stretch: (u8, u8),
    pub advances_xy: bool,
}

impl Image {
    /// Used for debugging only
    pub fn as_pbm(&self) -> Vec<u8> {
        let dim = format!("{} {}", self.width, self.height);
        let dimbytes = dim.as_bytes();

        let mut data: Vec<u8> = vec![0x50, 0x34, 0x0A];

        for b in dimbytes {
            data.push(*b)
        }

        data.extend(self.pixels.clone());
        data
    }

    /// Always returns 1 pixel per byte.
    pub fn as_grayscale(&self) -> Vec<u8> {
        if self.pixel_type == PixelType::MonochromeByte {
            return self.pixels.clone();
        }
        let mut bytes = Vec::<u8>::new();

        //number of bytes we need to use for the last column of each row of data
        let mut padding = self.width % 8;
        if padding == 0 {
            padding = 8;
        }
        let mut col = 0;

        for byte in &self.pixels {
            col += 8;
            if col >= self.width {
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

        bytes
    }

    pub fn from_raster_data(data: &Vec<u8>) -> Option<Image> {
        if data.len() < 8 {
            return None;
        };

        let a = *data.get(0).unwrap();
        let bx = *data.get(1).unwrap();
        let by = *data.get(2).unwrap();
        let c = *data.get(3).unwrap();
        let x1 = *data.get(4).unwrap();
        let x2 = *data.get(5).unwrap();
        let y1 = *data.get(6).unwrap();
        let y2 = *data.get(7).unwrap();
        let mut width = x1 as u32 + x2 as u32 * 256;
        let mut height = y1 as u32 + y2 as u32 * 256;

        let pixel_type = match a {
            48 => PixelType::Monochrome(c),
            52 => PixelType::MultipleTone(c, 1),
            _ => PixelType::Unknown,
        };

        let stretch = (bx, by);

        let pixels = if bx > 1 || by > 1 {
            let (w, h, px) =
                scale_pixels(&data[8..], width as usize, height as usize, bx > 1, by > 1);
            width = w;
            height = h;
            px
        } else {
            data[8..].to_vec()
        };

        println!("SCALE x{} y{}", bx, by);

        Some(Image {
            pixels,
            width,
            height,
            pixel_type,
            stretch,
            advances_xy: true,
        })
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
        let b = *data.get(3).unwrap();
        let x1 = *data.get(4).unwrap();
        let x2 = *data.get(5).unwrap();
        let y1 = *data.get(6).unwrap();
        let y2 = *data.get(7).unwrap();
        let _c = *data.get(8).unwrap();
        let width = x1 as u32 + x2 as u32 * 256;
        let height = y1 as u32 + y2 as u32 * 256;

        //b (above) specifies number of color data stored,
        // we are ignoring this for now if b > 1
        // [byte(color) bytes(capacity)] [byte(color) bytes(capacity)]
        let pixel_type = match a {
            48 => PixelType::Monochrome(1),
            52 => PixelType::MultipleTone(1, b),
            _ => PixelType::Unknown,
        };

        let stretch = (1, 1);

        let pixels = data[9..].to_vec();

        Some((
            ImageRef { kc1, kc2, storage },
            Image {
                pixels,
                width,
                height,
                pixel_type,
                stretch,
                advances_xy: true,
            },
        ))
    }

    pub fn from_column_data(data: &Vec<u8>) -> Option<Image> {
        if data.len() < 8 {
            return None;
        };

        let a = *data.get(0).unwrap();
        let bx = *data.get(1).unwrap();
        let by = *data.get(2).unwrap();
        let c = *data.get(3).unwrap();
        let x1 = *data.get(4).unwrap();
        let x2 = *data.get(5).unwrap();
        let y1 = *data.get(6).unwrap();
        let y2 = *data.get(7).unwrap();
        let mut width = x1 as u32 + x2 as u32 * 256;
        let mut height = y1 as u32 + y2 as u32 * 256;

        let pixel_type = match a {
            48 => PixelType::Monochrome(c),
            52 => PixelType::MultipleTone(c, 1),
            _ => PixelType::Unknown,
        };

        let stretch = (bx, by);

        let pixels = if bx > 1 || by > 1 {
            let (w, h, px) =
                scale_pixels(&data[8..], width as usize, height as usize, bx > 1, by > 1);
            width = w;
            height = h;
            px
        } else {
            data[8..].to_vec()
        };

        Some(Image {
            pixels,
            width,
            height,
            pixel_type,
            stretch,
            advances_xy: false,
        })
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
        let b = *data.get(3).unwrap();
        let x1 = *data.get(4).unwrap();
        let x2 = *data.get(5).unwrap();
        let y1 = *data.get(6).unwrap();
        let y2 = *data.get(7).unwrap();
        let width = x1 as u32 + x2 as u32 * 256;
        let height = y1 as u32 + y2 as u32 * 256;

        //b (above) specifies number of color data stored,
        // we are ignoring this for now if b > 1
        // [byte(color) bytes(capacity)] [byte(color) bytes(capacity)]
        let pixel_type = match a {
            48 => PixelType::Monochrome(1),
            52 => PixelType::MultipleTone(1, b),
            _ => PixelType::Unknown,
        };

        let stretch = (1, 1);

        let pixels = data[8..].to_vec();

        Some((
            ImageRef { kc1, kc2, storage },
            Image {
                pixels,
                width,
                height,
                pixel_type,
                stretch,
                advances_xy: false,
            },
        ))
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
    final_width: usize,
    final_height: usize,
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

    let rot = rotate_90_clockwise(bytes, final_height, final_width);
    let flip = flip_right_to_left(rot, final_width, final_height);

    if stretch.0 > 1 || stretch.1 > 1 {
        scale_pixels(
            &flip,
            final_width,
            final_height,
            stretch.0 > 1,
            stretch.1 > 1,
        )
    } else {
        (final_width as u32, final_height as u32, flip)
    }
}

fn rotate_90_clockwise(data: Vec<u8>, width: usize, height: usize) -> Vec<u8> {
    let mut result = vec![0; data.len()];

    for y in 0..height {
        for x in 0..width {
            let src_index = y * width + x;
            let dest_x = height - 1 - y;
            let dest_y = x;
            let dest_index = dest_y * height + dest_x; // Note: height is used for new row length
            result[dest_index] = data[src_index];
        }
    }

    result
}

fn flip_right_to_left(data: Vec<u8>, width: usize, height: usize) -> Vec<u8> {
    let mut result = vec![0; data.len()];

    for y in 0..height {
        for x in 0..width {
            let src_index = y * width + x;
            let dest_x = width - 1 - x; // Flip the x-coordinate
            let dest_index = y * width + dest_x; // Calculate the destination index
            result[dest_index] = data[src_index];
        }
    }

    result
}

pub fn scale_pixels(
    bytes: &[u8],
    original_width: usize,
    original_height: usize,
    scale_x: bool,
    scale_y: bool,
) -> (u32, u32, Vec<u8>) {
    let new_width = if scale_x {
        original_width * 2
    } else {
        original_width
    };
    let new_height = if scale_y {
        original_height * 2
    } else {
        original_height
    };

    let mut scaled_bytes = Vec::with_capacity(new_width * new_height);

    for y in 0..original_height {
        let row_start = y * original_width;
        let row_end = row_start + original_width;
        let row = &bytes[row_start..row_end];
        let mut scaled_row = Vec::with_capacity(new_width);

        for &pixel in row {
            if scale_x {
                scaled_row.push(pixel);
                scaled_row.push(pixel);
            } else {
                scaled_row.push(pixel);
            }
        }

        scaled_bytes.extend_from_slice(&scaled_row);
        if scale_y {
            scaled_bytes.extend_from_slice(&scaled_row);
        }
    }

    (new_width as u32, new_height as u32, scaled_bytes)
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
    Code2D(Code2D),
    Barcode(Barcode),
    Image(Image),
    Rectangle(Rectangle),
    Line(Line),
}
