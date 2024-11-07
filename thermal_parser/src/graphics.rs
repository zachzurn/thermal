use crate::context::{HumanReadableInterface, RenderColors};
use crate::text::TextSpan;
use crate::util::bitflags_lsb;

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

    /// Get a copy of this color with a new alpha
    pub fn with_alpha(mut self, a: u8) -> Self {
        RGBA {
            r: self.r,
            g: self.g,
            b: self.g,
            a,
        }
    }

    /// Blends a foreground color onto this color
    pub fn blend_foreground(&mut self, color: &Self) {
        self.blend_foreground_with_alpha(&color, &color.a);
    }

    /// Blends a foreground color onto this color.
    /// Uses the provided alpha instead of the one in
    /// the color.
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

#[derive(Clone, Debug, Copy)]
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
    /// Creates a vec with rgb data encoded as a contiguous
    /// vec of bytes. Useful for external libraries like png.
    pub fn as_rgb_u8(&self) -> Vec<u8> {
        let mut rgb_bytes = Vec::with_capacity(self.pixels.len() * 3);

        for pixel in self.pixels.iter() {
            rgb_bytes.push(pixel.r);
            rgb_bytes.push(pixel.g);
            rgb_bytes.push(pixel.b);
        }

        rgb_bytes
    }
}

impl GraphicsCommand {
    /// Parses column format into a single GraphicsCommand(Image).
    ///
    /// GraphicsCommand(Error) can also be returned from this function
    /// in order to provide meaningful error messages for corrupt data
    pub fn image_from_column_bytes_single_color(
        width: u32,
        height: u32,
        stretch: (u8, u8),
        color: &RGBA,
        flow: ImageFlow,
        data: &[u8],
    ) -> GraphicsCommand {
        let raster = column_to_bytes(data, width, height);
        Self::image_from_raster_bytes_single_color(
            width, height, stretch, color, flow, &raster, false,
        )
    }

    /// Parses column format that has multiple color layers into
    /// a single GraphicsCommand(Image).
    ///
    /// First decodes the column format into the common raster format.
    ///
    /// Color layer data has one byte to indicate the color and the
    /// rest of the bytes are bit encoded pixel data.
    ///
    /// GraphicsCommand(Error) can also be returned from this function
    /// in order to provide meaningful error messages for corrupt data
    pub fn image_from_column_bytes_multi_color(
        width: u32,
        height: u32,
        stretch: (u8, u8),
        num_colors: u8,
        render_colors: &RenderColors,
        flow: ImageFlow,
        data: &[u8],
    ) -> GraphicsCommand {
        let bytes_per_layer = ((width as usize / 8) * height as usize) + 1;

        //Ensure there are enough bytes to construct the final image
        if data.len() != (bytes_per_layer + 1) * num_colors as usize {
            return GraphicsCommand::Error("Not enough data to parse raster image".into());
        }

        let mut image_layers = vec![];

        for layer in 0..=num_colors as usize {
            let layer_start = layer * bytes_per_layer;
            let color_number = data[layer_start];
            let color = render_colors.color_for_number(color_number);
            let image_data = &data[layer_start..bytes_per_layer - 1];
            let raster = column_to_bytes(image_data, width, height);
            let layer = Self::image_from_raster_bytes_single_color(
                width, height, stretch, color, flow, &raster, false,
            );

            match layer {
                GraphicsCommand::Image(image) => image_layers.push(image),
                GraphicsCommand::Error(message) => return GraphicsCommand::Error(message),
                _ => {}
            }
        }

        let merge = merge_image_layers(&mut image_layers);

        match merge {
            Ok(merge) => GraphicsCommand::Image(merge),
            Err(e) => GraphicsCommand::Error(e.to_string()),
        }
    }

    /// Parses column format that has a single color layer into
    /// a single GraphicsCommand(Image).
    ///
    /// GraphicsCommand(Error) can also be returned from this function
    /// in order to provide meaningful error messages for corrupt data
    pub fn image_from_raster_bytes_single_color(
        width: u32,
        height: u32,
        stretch: (u8, u8),
        color: &RGBA,
        flow: ImageFlow,
        data: &[u8],
        process_as_bits: bool,
    ) -> GraphicsCommand {
        let unpacked = if process_as_bits {
            unpack_bytes(data, width, height)
        } else {
            data.to_vec()
        };

        //Ensure there are enough bytes to construct the final image
        if unpacked.len() != (width * height) as usize {
            return GraphicsCommand::Error(format!(
                "Not enough data to parse single color raster image expected: {} got: {}",
                unpacked.len(),
                data.len()
            ));
        }

        let (w, h, raw_pixels) = if stretch.0 > 1 || stretch.1 > 1 {
            scale_pixels(&unpacked, width as u32, height as u32, stretch.0, stretch.1)
        } else {
            (width, height, unpacked)
        };

        println!(
            "raster after scale, was w{} h{} len{} now w{} h{} len{}",
            width,
            height,
            data.len(),
            w,
            h,
            raw_pixels.len(),
        );

        let mut pixels = Vec::with_capacity(w as usize * h as usize);

        for i in 0..raw_pixels.len() {
            pixels.push(color.with_alpha(raw_pixels[i]));
        }

        GraphicsCommand::Image(Image {
            pixels,
            x: 0,
            y: 0,
            w,
            h,
            flow,
            upside_down: false,
        })
    }

    /// Parses column format that has multiple color layers into
    /// a single GraphicsCommand(Image).
    ///
    /// Color layer data has one byte to indicate the color and the
    /// rest of the bytes are bit encoded pixel data.
    ///
    /// GraphicsCommand(Error) can also be returned from this function
    /// in order to provide meaningful error messages for corrupt data
    pub fn image_from_raster_bytes_multi_color(
        width: u32,
        height: u32,
        stretch: (u8, u8),
        num_colors: u8,
        render_colors: &RenderColors,
        flow: ImageFlow,
        data: &[u8],
        process_as_bits: bool,
    ) -> GraphicsCommand {
        let bytes_per_layer = if process_as_bits {
            ((width / 8) as usize * height as usize) + 1
        } else {
            (width as usize * height as usize) + 1
        };

        //Ensure there are enough bytes to construct the final image
        if data.len() != bytes_per_layer * num_colors as usize {
            return GraphicsCommand::Error(
                "Not enough data to parse raster image multi color".into(),
            );
        }

        let mut image_layers = vec![];

        for layer in 0..=num_colors as usize {
            let layer_start = layer * bytes_per_layer;
            let layer_end = layer_start + bytes_per_layer - 1;
            let color_number = data[layer_start];
            let color = render_colors.color_for_number(color_number);
            let image_data = &data[layer_start + 1..layer_end];

            let layer = Self::image_from_raster_bytes_single_color(
                width,
                height,
                stretch,
                color,
                flow,
                &image_data,
                true,
            );

            match layer {
                GraphicsCommand::Image(image) => image_layers.push(image),
                GraphicsCommand::Error(message) => return GraphicsCommand::Error(message),
                _ => {}
            }
        }

        let merge = merge_image_layers(&mut image_layers);

        match merge {
            Ok(merge) => GraphicsCommand::Image(merge),
            Err(e) => GraphicsCommand::Error(e.to_string()),
        }
    }
}

/// This function is used to combine multiple colors into one image.
/// Merges a Vec of images into one using the first image as the base.
///
/// ESC/POS Images are always encoded with on/off bits and need to have
/// color introduced.
///
pub fn merge_image_layers(layers: &Vec<Image>) -> Result<Image, &'static str> {
    if layers.is_empty() {
        return Err("No layers to merge".into());
    }
    if layers.len() == 1 {
        return Ok(layers[0].clone());
    }

    let mut image = layers[0].clone();

    //For the moment, we only merge if all images have the same width and height
    for merge_img in layers.iter().skip(1) {
        if merge_img.w > image.w || merge_img.h > image.h {
            return Err("Can't merge image layers with different w and h".into());
        }

        for i in 0..merge_img.pixels.len() {
            image.pixels[i].blend_foreground(&merge_img.pixels[i]);
        }
    }

    Ok(image)
}

/// Unpacks bits into bytes and makes sure that extra padding
/// is added for widths that are not divisible by eight.
fn unpack_bytes(pixels: &[u8], width: u32, height: u32) -> Vec<u8> {
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
                bytes.push(if *byte & 1 << (7 - n) != 0 { 255 } else { 0 });
            }
            col = 0;
        } else {
            bytes.push(if *byte & 1 << 7 != 0 { 255 } else { 0 });
            bytes.push(if *byte & 1 << 6 != 0 { 255 } else { 0 });
            bytes.push(if *byte & 1 << 5 != 0 { 255 } else { 0 });
            bytes.push(if *byte & 1 << 4 != 0 { 255 } else { 0 });
            bytes.push(if *byte & 1 << 3 != 0 { 255 } else { 0 });
            bytes.push(if *byte & 1 << 2 != 0 { 255 } else { 0 });
            bytes.push(if *byte & 1 << 1 != 0 { 255 } else { 0 });
            bytes.push(if *byte & 1 << 0 != 0 { 255 } else { 0 });
        }
    }

    bytes
}

/// Column format is weird and needs to have some special operations
/// done on the unpacked bytes to get the image in the correct orientation.
fn column_to_bytes(pixels: &[u8], width: u32, height: u32) -> Vec<u8> {
    let unpacked = unpack_bytes(pixels, height, width);
    let rot = rotate_90_clockwise(unpacked, height, width);
    flip_right_to_left(rot, width, height)
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

/// Images can often have a scale width and height factor
/// This is a dirty scaling that just copies bytes in the
/// x and y direction
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

/// Images that were added to storage can be
/// referenced with an ImageRef
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
