use crate::context::{HumanReadableInterface, RenderColors};
use crate::text::TextSpan;
use crate::util::{bitflags_lsb, bitflags_msb, parse_u16};

#[derive(Clone, Copy, Debug)]
pub struct RGBA {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl RGBA {
    pub fn blank() -> Self {
        RGBA {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        }
    }

    pub fn with_alpha(mut self, a: u8) -> Self {
        RGBA {
            r: self.r,
            g: self.g,
            b: self.g,
            a,
        }
    }

    pub fn blend_foreground(&mut self, color: &Self) {
        self.blend_foreground_with_alpha(&color, &color.a);
    }

    pub fn blend_foreground_with_alpha(&mut self, color: &Self, alpha: &u8) {
        //Fully opaque, no calculations needed
        if alpha == &255 {
            self.r = color.r;
            self.g = color.g;
            self.b = color.b;
            self.a = 255;
        }

        //Fully transparent, no change
        if alpha == &0 {
            return;
        }

        let alpha = *alpha as f32 / 255.0;
        self.r = (color.r as f32 * alpha + self.r as f32 * (1.0 - alpha)).round() as u8;
        self.g = (color.g as f32 * alpha + self.g as f32 * (1.0 - alpha)).round() as u8;
        self.b = (color.b as f32 * alpha + self.b as f32 * (1.0 - alpha)).round() as u8;
    }
}

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
    pub pixels: Vec<RGBA>,
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
    pub flow: ImageFlow,
    pub upside_down: bool,
}

impl Image {
    pub fn from_raster_bytes(
        color: &RGBA,
        width: u32,
        height: u32,
        stretch: (u8, u8),
        data: &[u8],
    ) -> Image {
        let (w, h, raw_pixels) = if stretch.0 > 1 || stretch.1 > 1 {
            scale_pixels(
                &data[8..],
                width as u32,
                height as u32,
                stretch.0,
                stretch.1,
            )
        } else {
            (width, height, data.to_vec())
        };

        let mut pixels = Vec::with_capacity(data.len());

        for i in 0..data.len() {
            pixels[i] = color.with_alpha(raw_pixels[i]);
        }

        Image {
            pixels,
            x: 0,
            y: 0,
            w,
            h,
            flow: ImageFlow::Block,
            upside_down: false,
        }
    }

    pub fn from_column_bytes(color: &RGBA, width: u32, height: u32, data: &Vec<u8>) -> Vec<RGBA> {
        let mut pixels = Vec::<RGBA>::new();
        let on = color.with_alpha(255);
        let off = RGBA::blank();

        //number of bytes we need to use for the last column of each row of data
        let mut padding = width % 8;
        if padding == 0 {
            padding = 8;
        }
        let mut col = 0;

        for byte in data {
            col += 8;
            if col >= width {
                for n in 0..padding {
                    pixels.push(if *byte & 1 << (7 - n) != 0 { off } else { on });
                }
                col = 0;
            } else {
                let values = bitflags_msb(byte);

                pixels.push(if values.0 { off } else { on });
                pixels.push(if values.1 { off } else { on });
                pixels.push(if values.2 { off } else { on });
                pixels.push(if values.3 { off } else { on });
                pixels.push(if values.4 { off } else { on });
                pixels.push(if values.5 { off } else { on });
                pixels.push(if values.6 { off } else { on });
                pixels.push(if values.7 { off } else { on });
            }
        }

        pixels
    }

    pub fn from_raster_data(data: &Vec<u8>, render_colors: &RenderColors) -> Option<Image> {
        if data.len() < 8 {
            return None;
        };

        let a = *data.get(0).unwrap();
        let bx = *data.get(1).unwrap();
        let by = *data.get(2).unwrap();

        //TODO parse into a color
        let c = *data.get(3).unwrap();

        let mut width = parse_u16(data, 4) as u32;
        let mut height = parse_u16(data, 6) as u32;
        let stretch = (bx, by);

        let img = Self::from_raster_bytes(
            render_colors.color_for_number(c),
            width,
            height,
            stretch,
            &data[8..],
        );
        Some(img)
    }

    pub fn from_raster_data_with_ref(
        data: &Vec<u8>,
        storage: ImageRefStorage,
        render_colors: &RenderColors,
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
        let stretch = (1, 1);

        let img = Self::from_raster_bytes(
            render_colors.color_for_number(c),
            width,
            height,
            stretch,
            &data[9..],
        );
        Some((ImageRef { kc1, kc2, storage }, img))
    }

    pub fn from_column_data(data: &Vec<u8>, render_colors: &RenderColors) -> Option<Image> {
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

        let decoded = column_to_raster(&data[8..], width, height);

        let img = Self::from_raster_bytes(
            render_colors.color_for_number(c),
            decoded.0,
            decoded.1,
            stretch,
            &decoded.2,
        );

        Some(img)
    }

    pub fn from_column_data_with_ref(
        data: &Vec<u8>,
        storage: ImageRefStorage,
        render_colors: &RenderColors,
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

        let decoded = column_to_raster(&data[8..], width, height);

        let img = Self::from_raster_bytes(
            render_colors.color_for_number(1),
            decoded.0,
            decoded.1,
            stretch,
            &decoded.2,
        );

        Some((ImageRef { kc1, kc2, storage }, img))
    }
}

//TODO see if we can combine bit decoding functionality in here instead of duplicating
pub fn column_to_raster(pixels: &[u8], final_width: u32, final_height: u32) -> (u32, u32, Vec<u8>) {
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
    let mut flip = flip_right_to_left(rot, final_width, final_height);

    (final_width, final_height, flip)
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
