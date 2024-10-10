extern crate fontdue;
extern crate png;
extern crate textwrap;

use std::rc::Rc;
use fontdue::layout::CharacterData;
use thermal_parser::graphics::{Image, Rectangle};
use thermal_parser::text::TextSpan;

const THRESHOLD: u8 = 120;
const SCALE_THRESHOLD: u8 = 140;
const BASE_FONT_SIZE: f32 = 21.0;

pub struct FontFamily {
    pub regular: Rc<fontdue::Font>,
    pub bold: Rc<fontdue::Font>,
    pub italic: Rc<fontdue::Font>,
    pub bold_italic: Rc<fontdue::Font>,
}

/// A simple image renderer designed for thermal image generation
/// This allows for an image with a fixed width that can grow in height
/// to accommodate sets of pixels being pushed at arbitrary x and y values
pub struct ThermalImage {
    bytes: Vec<u8>,
    pub width: u32,
    pub font: FontFamily,
    pub auto_grow: bool,
    pub allow_greyscale: bool,
    pub debug: bool,
}

impl ThermalImage {
    pub fn new(width: u32) -> Self {
        let regular = fontdue::Font::from_bytes(
            include_bytes!("../../resources/fonts/JetBrainsMonoNL-Medium.ttf") as &[u8],
            fontdue::FontSettings::default(),
        )
        .unwrap();
        let bold = fontdue::Font::from_bytes(
            include_bytes!("../../resources/fonts/JetBrainsMonoNL-Bold.ttf") as &[u8],
            fontdue::FontSettings::default(),
        )
        .unwrap();
        let italic = fontdue::Font::from_bytes(
            include_bytes!("../../resources/fonts/JetBrainsMonoNL-MediumItalic.ttf") as &[u8],
            fontdue::FontSettings::default(),
        )
        .unwrap();
        let bold_italic = fontdue::Font::from_bytes(
            include_bytes!("../../resources/fonts/JetBrainsMonoNL-BoldItalic.ttf") as &[u8],
            fontdue::FontSettings::default(),
        )
        .unwrap();

        let font = FontFamily {
            regular: Rc::from(regular),
            bold: Rc::from(bold),
            italic: Rc::from(italic),
            bold_italic: Rc::from(bold_italic),
        };

        Self {
            bytes: Vec::<u8>::new(),
            font,
            width,
            allow_greyscale: true,
            auto_grow: true,
            debug: true
        }
    }

    pub fn get_font(&self, span: &TextSpan) -> Rc<fontdue::Font> {
        if span.bold && span.italic {
            return self.font.bold_italic.clone();
        }
        if span.bold {
            return self.font.bold.clone();
        }
        if span.italic {
            return self.font.italic.clone();
        }
        self.font.regular.clone()
    }

    pub fn rotate_90(&mut self) {
        let w = self.width as usize;
        let h = self.get_height() as usize;
        let mut rotated_image = vec![0; (w * h) as usize];

        for y in 0..h {
            for x in 0..w {
                rotated_image[x * h + (h - 1 - y)] = self.bytes[y * w + x];
            }
        }

        self.bytes = rotated_image;
        self.width = h as u32;
    }

    pub fn rotate_180(&mut self) {
        let w = self.width as usize;
        let h = self.get_height() as usize;
        let mut rotated_image = vec![0; (w * h) as usize];

        for y in 0..h {
            for x in 0..w {
                rotated_image[(h - 1 - y) * w + (w - 1 - x)] = self.bytes[y * w + x];
            }
        }

        self.bytes = rotated_image;
    }

    pub fn rotate_270(&mut self) {
        let w = self.width as usize;
        let h = self.get_height() as usize;
        let mut rotated_image = vec![0; (w * h) as usize];

        for y in 0..h {
            for x in 0..w {
                rotated_image[(w - 1 - x) * h + y] = self.bytes[y * w + x];
            }
        }

        self.bytes = rotated_image;
        self.width = h as u32;
    }

    //Setting the width clears any bytes
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.bytes = Vec::<u8>::new();
    }

    pub fn reset(&mut self) {
        self.bytes.clear();
        self.bytes.shrink_to(0);
    }

    pub fn draw_rect(&mut self, x: u32, y: u32, w: u32, h: u32) {
        self.put_pixels(x, y, w, h, vec![0u8; (w * h) as usize], false, true);
    }

    pub fn draw_border(bytes: &mut Vec<u8>, width: u32, height: u32, border_value: u8) {
        // Top row (y = 0)
        for x in 0..width {
            let idx = x as usize; // Top row index
            bytes[idx] = border_value;
        }

        // Bottom row (y = height - 1)
        for x in 0..width {
            let idx = ((height - 1) * width + x) as usize; // Bottom row index
            bytes[idx] = border_value;
        }

        // Left column and right column
        for y in 0..height {
            let left_idx = (y * width) as usize; // Left column index
            let right_idx = (y * width + (width - 1)) as usize; // Right column index

            bytes[left_idx] = border_value; // Left border pixel
            bytes[right_idx] = border_value; // Right border pixel
        }
    }

    fn render_char(char: char, width: u32, height: u32, final_width: u32, final_height: u32, font: Rc<fontdue::Font>, font_size: f32) -> Option<(Vec<u8>, u32, u32)> {
        let mut bytes = vec![0u8; width as usize * height as usize];
        let font_metrics = font.horizontal_line_metrics(font_size).unwrap();
        let baseline = f32::ceil(font_metrics.ascent + font_metrics.descent);
        let (metrics, char_bitmap) = font.rasterize(char, font_size);
        let glyph_index = font.lookup_glyph_index(char);
        let char_data = CharacterData::classify(char, glyph_index);
        if char_data.is_control() { return None; }

        let y_offset =
            f32::ceil((baseline - metrics.bounds.height) + (-1.0 * metrics.bounds.ymin)) as u32;
        
        let x_offset = 0;

        // Add text to empty char bitmap
        if metrics.width > 0 {
            for (y, row) in char_bitmap.chunks(metrics.width).enumerate() {
                for (x, &pixel) in row.iter().enumerate() {
                    // Calculate target x and y in the final bitmap
                    let target_x = (x as u32).saturating_add(x_offset); // Ensure x_offset is positive
                    let target_y = (y as u32).saturating_add(y_offset); // Ensure y_offset is positive

                    // Check if the target position is within the bounds of the larger bitmap
                    if target_x < width && target_y < height {
                        // Calculate the index in the larger bitmap
                        let idx = (target_y * width + target_x) as usize;

                        // Safely place the pixel into the larger bitmap
                        bytes[idx] = pixel;
                    }
                }
            }
        }
        
        // Scale if needed
        if final_width > width || final_height > height {
            return Some((ThermalImage::scale_bitmap(
                &bytes,
                width,
                height,
                final_width,
                final_height,
            ), final_width, final_height));
        }

        Some((bytes, final_width, final_height))
    }

    pub fn render_span(&mut self, x_offset: u32, max_height: u32, span: &TextSpan) {
        if span.dimensions.is_none() { return; }
        let dimensions = span.dimensions.as_ref().unwrap();
        let font = self.get_font(span);
        let mut cur_x = dimensions.x + x_offset;
        let y_offset = max_height - span.character_height;
        
        for char in span.text.chars() {
            let char_bitmap = ThermalImage::render_char(char, span.base_character_width, span.base_character_height, span.character_width, span.character_height, font.clone(), BASE_FONT_SIZE);

            if let Some(mut bitmap) = char_bitmap {
                if bitmap.1 == 0 || bitmap.2 == 0 { continue; }
                
                if self.debug {
                    ThermalImage::draw_border(&mut bitmap.0, bitmap.1, bitmap.2, 255);
                }

                self.put_pixels(
                    cur_x,
                    dimensions.y + y_offset,
                    bitmap.1,
                    bitmap.2,
                    bitmap.0,
                    true,
                    true,
                );
            }

            cur_x += span.character_width;
        }

        if span.underline > 0 {
            //TODO
            //self.draw_rect(under_x, under_y, dimensions.w, span.underline);
        }

        if span.strikethrough > 0 {
            //TODO
            // let strike_y = dimensions.y + ((font_metrics.ascent * span.stretch_height) / 2.0) as u32;
            // let strike_x = dimensions.x + x_offset;
            //
            // self.draw_rect(
            //     strike_x,
            //     strike_y - span.strikethrough,
            //     dimensions.w,
            //     span.strikethrough,
            // );
        }

        if span.inverted {
            self.invert_pixels(dimensions.x + x_offset, dimensions.y + y_offset, dimensions.w, dimensions.h);
        }

        if span.upside_down {
            self.flip_pixels(dimensions.x + x_offset, dimensions.y + y_offset, dimensions.w, dimensions.h);
        }
    }

    pub fn scale_bitmap(
        bitmap: &Vec<u8>,
        width: u32,
        height: u32,
        sw: u32,
        sh: u32,
    ) -> Vec<u8> {
        // Create a new scaled bitmap
        let mut scaled_bitmap = vec![0u8; (sw * sh) as usize];

        // Calculate scaling factors for width and height
        let x_ratio = width as f32 / sw as f32;
        let y_ratio = height as f32 / sh as f32;

        // Loop over every pixel in the scaled bitmap
        for sy in 0..sh {
            for sx in 0..sw {
                // Find the corresponding pixel in the original bitmap
                let src_x = (sx as f32 * x_ratio) as u32;
                let src_y = (sy as f32 * y_ratio) as u32;

                // Ensure the indices are within bounds (should already be safe with casting)
                let src_index = (src_y * width + src_x) as usize;

                // Calculate the index in the scaled bitmap
                let dst_index = (sy * sw + sx) as usize;

                // Safely copy the pixel value if within bounds
                if src_index < bitmap.len() && dst_index < scaled_bitmap.len() {
                    scaled_bitmap[dst_index] = bitmap[src_index];
                }
            }
        }

        scaled_bitmap
    }

    pub fn invert_pixels(&mut self, x: u32, y: u32, width: u32, height: u32) {
        if x + width > self.width {
            return;
        };
        self.expand_to_height(y + height);

        let mut cur_y = y;
        let mut cur_x = x;

        for _ in y..y + height {
            let idx = cur_y * self.width + cur_x;
            for i in 0..width {
                self.bytes[idx as usize + i as usize] = 255 - self.bytes[idx as usize + i as usize];
                cur_x += 1;
            }
            cur_x = x;
            cur_y += 1;
        }
    }

    pub fn flip_pixels(&mut self, x: u32, y: u32, width: u32, height: u32) {
        if x + width > self.width {
            return;
        };
        self.expand_to_height(y + height);

        let mut sub_image = Vec::<u8>::with_capacity((width * height) as usize);

        for cur_y in y..y + height {
            let idx = cur_y * self.width + x;
            for cur_x in x..width {
                sub_image.push(self.bytes[idx as usize + cur_x as usize]);
            }
        }

        sub_image.reverse();

        self.put_pixels(x, y, width, height, sub_image, false, false);
    }

    pub fn put_rect(&mut self, rectangle: &Rectangle) {
        self.draw_rect(rectangle.x, rectangle.y, rectangle.w, rectangle.h);
    }

    pub fn put_render_img(&mut self, image: &Image) {
        self.put_pixels(
            image.x,
            image.y,
            image.w,
            image.h,
            image.as_grayscale(),
            false,
            true,
        );
        if image.upside_down {
            self.flip_pixels(image.x, image.y, image.w, image.h);
        }
    }

    /// Add pixels to the current canvas.
    /// Images that are too wide are always cropped.
    /// Images that are too tall auto grow the canvas
    /// unless auto_grow is set to false.
    /// invert will reverse black and white pixels.
    /// multiply will ensure that a white pixel does
    /// not overwrite a black existing pixel.
    pub fn put_pixels(
        &mut self,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        pixels: Vec<u8>,
        invert: bool,
        multiply: bool,
    ) -> bool {
        let mut cur_x = x;
        let mut cur_y = y;

        //Bad pixel data
        if width * height < pixels.len() as u32 {
            println!("Bad pixel data, not enough bytes");
            return false;
        }

        //Out of bounds
        let exceeds_w = x >= self.width;
        let exceeds_h = y >= self.get_height();

        //Completely out of bounds, unrenderable
        if exceeds_w || (exceeds_h && !self.auto_grow) {
            println!(
                "Exceeds ew{} eh{} w{} h{}",
                exceeds_w,
                exceeds_h,
                self.width,
                self.get_height()
            );
            return false;
        }

        //Width can never grow, height can grow is auto_grow = true
        let needs_crop_w = x + width > self.width;
        let needs_crop_h = !self.auto_grow && (y + width > self.get_height());

        let (final_width, final_height, final_pixels) = if needs_crop_w || needs_crop_h {
            let max_width = self.width - x;
            let max_height = if self.get_height() <= y {
                0
            } else {
                self.get_height() - y
            };

            Self::crop_pixels(
                &pixels,
                width,
                height,
                max_width,
                max_height,
                !self.auto_grow,
            )
        } else {
            (width, height, pixels)
        };

        self.expand_to_height(y + final_height);

        if multiply {
            for pixel in final_pixels {
                let idx = cur_y * self.width + cur_x;

                //ensure black or white only
                let pixel = if !self.allow_greyscale {
                    if pixel < THRESHOLD {
                        0
                    } else {
                        255
                    }
                } else {
                    pixel
                };

                self.bytes[idx as usize] = if invert {
                    u8::min(255 - pixel, self.bytes[idx as usize])
                } else {
                    u8::min(pixel, self.bytes[idx as usize])
                };
                if cur_x == x + final_width - 1 {
                    cur_x = x;
                    cur_y += 1;
                } else {
                    cur_x += 1;
                }
            }
        } else {
            for pixel in final_pixels {
                let idx = cur_y * self.width + cur_x;
                self.bytes[idx as usize] = if invert { 255 - pixel } else { pixel };
                if cur_x == x + final_width - 1 {
                    cur_x = x;
                    cur_y += 1;
                } else {
                    cur_x += 1;
                }
            }
        }

        true
    }

    pub fn crop_pixels(
        pixels: &Vec<u8>,
        width: u32,
        height: u32,
        max_width: u32,
        max_height: u32,
        crop_height: bool,
    ) -> (u32, u32, Vec<u8>) {
        let new_width = if width > max_width { max_width } else { width };
        let new_height = if crop_height {
            if height > max_height {
                max_height
            } else {
                height
            }
        } else {
            height
        };

        let mut cropped_pixels = Vec::with_capacity((new_width * new_height) as usize);

        for y in 0..new_height {
            let row_start = y * width;
            let row = &pixels[row_start as usize..row_start as usize + new_width as usize];
            cropped_pixels.extend_from_slice(row);
        }

        // Return the new dimensions and the cropped pixels
        (new_width, new_height, cropped_pixels)
    }

    pub fn get_height(&self) -> u32 {
        if self.width == 0 {
            0
        } else {
            (self.bytes.len() / self.width as usize) as u32
        }
    }

    pub fn expand_to_height(&mut self, height: u32) {
        let len = (self.width * height) as usize;
        let cur_len = self.bytes.len();
        if cur_len >= len {
            return;
        }
        let to_add = len - cur_len;

        for _ in 0..to_add {
            self.bytes.push(255u8);
        }
    }

    pub fn add_top_margin(&mut self, height: u32) {
        self.bytes
            .splice(0..0, vec![255u8; (self.width * height) as usize]);
    }

    pub fn expand_to_width(&mut self, new_width: u32) {
        let old_width = self.width;
        if new_width < old_width {
            return;
        };
        let height = self.get_height();
        let left = ((new_width - old_width) as f32 / 2.0).floor() as u32;
        let right = new_width - (left + old_width);

        self.bytes
            .try_reserve((height as usize * new_width as usize - self.bytes.len()) as usize)
            .unwrap();

        let mut insert_idx = 0;

        let left_bytes = vec![255u8; left as usize];
        let right_bytes = vec![255u8; right as usize];

        for _ in 0..height {
            self.bytes
                .splice(insert_idx..insert_idx, left_bytes.clone());
            insert_idx += left as usize + old_width as usize;

            self.bytes
                .splice(insert_idx..insert_idx, right_bytes.clone());
            insert_idx += right as usize;
        }

        self.width = new_width;
    }

    pub fn copy(&mut self) -> (u32, u32, Vec<u8>) {
        let pixels = self.bytes.clone();
        let w = self.width;
        let h = self.get_height();
        (w, h, pixels)
    }

    // empty the pixels
    pub fn empty(&mut self) {
        self.bytes.clear()
    }
}


