extern crate fontdue;
extern crate png;

use crate::renderer::DebugProfile;
use fontdue::layout::CharacterData;
use std::mem;
use std::rc::Rc;
use thermal_parser::context::Font;
use thermal_parser::graphics::{Image, Rectangle, RGBA};
use thermal_parser::text::TextSpan;

const SIZE_TO_FONT_RATIO: f32 = 1.68;
const SIZE_TO_BASELINE_RATIO: f32 = 0.0315;

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
    bytes: Vec<RGBA>,
    pub width: u32,
    pub font: FontFamily,
    pub auto_grow: bool,
    pub debug_profile: DebugProfile,
    pub font_size: f32,
    pub paper_color: RGBA,
    pub text_debug_color: RGBA,
    pub baseline_debug_color: RGBA,
    pub image_debug_color: RGBA,
    pub errors: Vec<String>,
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
            errors: vec![],
            bytes: Vec::<RGBA>::new(),
            font,
            width,
            auto_grow: true,
            debug_profile: DebugProfile {
                text: false,
                image: false,
                page: false,
            },
            text_debug_color: RGBA {
                r: 98,
                g: 224,
                b: 89,
                a: 255,
            },
            baseline_debug_color: RGBA {
                r: 110,
                g: 255,
                b: 185,
                a: 255,
            },
            image_debug_color: RGBA {
                r: 216,
                g: 74,
                b: 252,
                a: 255,
            },
            paper_color: RGBA {
                r: 255,
                g: 255,
                b: 255,
                a: 255,
            },
            font_size: 12f32 * SIZE_TO_FONT_RATIO,
        }
    }

    fn get_font(&self, span: &TextSpan) -> Rc<fontdue::Font> {
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

    fn get_font_size(&self, font: &Font) -> (f32, f32) {
        match font {
            Font::B => (
                self.font_size * 0.8,
                (self.font_size * 0.8) * SIZE_TO_BASELINE_RATIO,
            ),
            Font::C => (
                self.font_size * 0.7,
                (self.font_size * 0.5) * SIZE_TO_BASELINE_RATIO,
            ),
            _ => (self.font_size, self.font_size * SIZE_TO_BASELINE_RATIO),
        }
    }

    pub fn rotate_90(&mut self) {
        let w = self.width as usize;
        let h = self.get_height() as usize;
        let mut rotated_image = vec![RGBA::blank(); (w * h) as usize];

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
        let mut rotated_image = vec![RGBA::blank(); (w * h) as usize];

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
        let mut rotated_image = vec![RGBA::blank(); (w * h) as usize];

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
        self.bytes = Vec::<RGBA>::new();
    }

    pub fn reset(&mut self) {
        self.bytes.clear();
        self.bytes.shrink_to(0);
    }

    pub fn draw_rect(&mut self, x: u32, y: u32, w: u32, h: u32, color: &RGBA) {
        self.put_pixels(x, y, w, h, vec![*color; (w * h) as usize], false, true);
    }

    pub fn draw_border(bytes: &mut Vec<RGBA>, width: u32, height: u32, color: &RGBA) {
        let bot_left = ((height - 1) * width) as usize;

        // Top and bottom border
        for x in 0..width as usize {
            let top_idx = x;
            let bot_idx = bot_left + x;

            bytes[top_idx].blend_foreground(color);
            bytes[bot_idx].blend_foreground(color);
        }

        // Left and right border
        for y in 0..height {
            let left_idx = (y * width) as usize; // Left column index
            let right_idx = (y * width + (width - 1)) as usize; // Right column index

            bytes[left_idx].blend_foreground(color); // Left border pixel
            bytes[right_idx].blend_foreground(color); // Right border pixel
        }
    }

    fn render_char(
        char: char,
        width: u32,
        height: u32,
        final_width: u32,
        final_height: u32,
        font: Rc<fontdue::Font>,
        font_size: f32,
        background_color: &RGBA,
        text_color: &RGBA,
    ) -> Option<(Vec<RGBA>, u32, u32)> {
        let w_scale = final_width / width;
        let h_scale = final_height / height;
        let scale = h_scale.max(w_scale);
        let scaled_font_size = font_size * scale as f32;

        //We render the char at full width/height, then scale down dimensions as needed
        let rendered_w = width * scale;
        let rendered_h = height * scale;

        let mut bytes = vec![*background_color; rendered_w as usize * rendered_h as usize];

        let (metrics, char_bitmap) = font.rasterize(char, scaled_font_size);
        let font_metrics = font.horizontal_line_metrics(scaled_font_size).unwrap();
        let baseline = f32::ceil(font_metrics.ascent + font_metrics.descent);
        let glyph_index = font.lookup_glyph_index(char);
        let char_data = CharacterData::classify(char, glyph_index);
        if char_data.is_control() {
            return None;
        }

        let y_offset =
            f32::ceil((baseline - metrics.bounds.height) + (-1.0 * metrics.bounds.ymin)) as u32;
        let x_offset = metrics.bounds.xmin.round().abs() as u32;
        //^ This can cut some chars off. We prefer to have thw whole char
        //show vs changing the font size ratio

        if metrics.width > 0 {
            for (y, row) in char_bitmap.chunks(metrics.width).enumerate() {
                for (x, &pixel) in row.iter().enumerate() {
                    let target_x = (x as u32).saturating_add(x_offset);
                    let target_y = (y as u32).saturating_add(y_offset);

                    if target_x < rendered_w && target_y < rendered_h {
                        let idx = (target_y * rendered_w + target_x) as usize;
                        bytes[idx].blend_foreground_with_alpha(&text_color, &pixel);
                    }
                }
            }
        }

        // Scale if needed
        if scale > 1 {
            return Some((
                ThermalImage::scale_bitmap(
                    &bytes,
                    rendered_w,
                    rendered_h,
                    final_width,
                    final_height,
                ),
                final_width,
                final_height,
            ));
        }

        Some((bytes, final_width, final_height))
    }

    pub fn render_span(&mut self, x_offset: u32, max_height: u32, span: &TextSpan) {
        if span.dimensions.is_none() {
            return;
        }
        let dimensions = span.dimensions.as_ref().unwrap();
        let font = self.get_font(span);
        let (font_size, baseline_ratio) = self.get_font_size(&span.font);
        let mut cur_x = dimensions.x + x_offset;
        let mut y_offset = max_height - span.character_height;

        if y_offset > 0 {
            //Calculate the actual y offset based on the preset baseline ratio
            let max_height_baseline = max_height as f32 * baseline_ratio;
            let span_baseline = span.character_height as f32 * baseline_ratio;
            y_offset = (max_height_baseline - span_baseline) as u32;
        }

        for char in span.text.chars() {
            let char_bitmap = ThermalImage::render_char(
                char,
                span.base_character_width,
                span.base_character_height,
                span.character_width,
                span.character_height,
                font.clone(),
                font_size,
                &span.background_color,
                &span.text_color,
            );

            if let Some(mut bitmap) = char_bitmap {
                if bitmap.1 == 0 || bitmap.2 == 0 {
                    continue;
                }

                if self.debug_profile.text {
                    //Use debug color 1
                    ThermalImage::draw_border(
                        &mut bitmap.0,
                        bitmap.1,
                        bitmap.2,
                        &self.text_debug_color.with_alpha(60),
                    );
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

        //Draw baseline
        if self.debug_profile.text {
            self.draw_rect(
                dimensions.x + x_offset,
                (dimensions.y + y_offset) + (span.character_height as f32 * baseline_ratio) as u32,
                dimensions.w,
                1,
                &self.baseline_debug_color.clone(),
            )
        }

        if span.underline > 0 {
            self.draw_rect(
                dimensions.x + x_offset,
                (dimensions.y + y_offset + 3)
                    + (span.character_height as f32 * baseline_ratio) as u32,
                dimensions.w,
                1,
                &span.text_color,
            )
        }

        if span.strikethrough > 0 {
            self.draw_rect(
                dimensions.x + x_offset,
                (dimensions.y + y_offset) + (span.character_height as f32 / 2.5) as u32,
                dimensions.w,
                span.strikethrough,
                &span.text_color,
            )
        }

        if span.upside_down {
            self.flip_pixels(
                dimensions.x + x_offset,
                dimensions.y + y_offset,
                dimensions.w,
                dimensions.h,
            );
        }
    }

    pub fn scale_bitmap(
        bitmap: &Vec<RGBA>,
        width: u32,
        height: u32,
        sw: u32,
        sh: u32,
    ) -> Vec<RGBA> {
        // Create a new scaled bitmap
        let mut scaled_bitmap = vec![RGBA::blank(); (sw * sh) as usize];

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

    pub fn flip_pixels(&mut self, x: u32, y: u32, width: u32, height: u32) {
        if x + width > self.width {
            return;
        };

        self.expand_to_height(y + height);

        // Vector to store rows
        let mut sub_image = Vec::<RGBA>::with_capacity((width * height) as usize);

        // Collect the sub-image row by row
        for cur_y in y..y + height {
            let start_idx = (cur_y * self.width + x) as usize;
            let end_idx = start_idx + width as usize;

            // Collect the current row and push it to the sub_image
            sub_image.extend_from_slice(&self.bytes[start_idx..end_idx]);
        }

        // Now reverse the rows to flip the image top-to-bottom
        let row_size = width as usize;
        for i in 0..(height as usize / 2) {
            let top_row_start = i * row_size;
            let bottom_row_start = (height as usize - i - 1) * row_size;

            // Swap rows by using a temporary buffer
            for j in 0..row_size {
                sub_image.swap(top_row_start + j, bottom_row_start + j);
            }
        }

        // Put the flipped pixels back into the image
        self.put_pixels(x, y, width, height, sub_image, false, false);
    }

    pub fn put_rect(&mut self, rectangle: &Rectangle, color: &RGBA) {
        self.draw_rect(rectangle.x, rectangle.y, rectangle.w, rectangle.h, color);
    }

    pub fn put_render_img(&mut self, image: &Image) {
        let mut pixels = image.pixels.clone();

        if self.debug_profile.image {
            ThermalImage::draw_border(&mut pixels, image.w, image.h, &self.image_debug_color);
        }

        self.put_pixels(image.x, image.y, image.w, image.h, pixels, false, true);

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
        pixels: Vec<RGBA>,
        invert: bool,
        multiply: bool,
    ) -> bool {
        let mut cur_x = x;
        let mut cur_y = y;

        //Bad pixel data
        if width * height < pixels.len() as u32 {
            self.errors
                .push("Bad pixel data, not enough bytes".to_string());
            return false;
        }

        //Out of bounds
        let exceeds_w = x >= self.width;
        let exceeds_h = y >= self.get_height();

        //Completely out of bounds, unrenderable
        if exceeds_w || (exceeds_h && !self.auto_grow) {
            self.errors.push(format!(
                "Image exceeded paper x{} y{} w{} h{} : exceeded: width? {} height? {}",
                x,
                y,
                self.width,
                self.get_height(),
                exceeds_w,
                exceeds_h,
            ));
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
            for pixel in &final_pixels {
                let idx = cur_y * self.width + cur_x;

                //TODO maybe use a multiply blend
                self.bytes[idx as usize].blend_foreground(pixel);

                if cur_x == x + final_width - 1 {
                    cur_x = x;
                    cur_y += 1;
                } else {
                    cur_x += 1;
                }
            }
        } else {
            for pixel in &final_pixels {
                let idx = cur_y * self.width + cur_x;
                self.bytes[idx as usize].blend_foreground(pixel);
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
        pixels: &Vec<RGBA>,
        width: u32,
        height: u32,
        max_width: u32,
        max_height: u32,
        crop_height: bool,
    ) -> (u32, u32, Vec<RGBA>) {
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
            self.bytes.push(self.paper_color);
        }
    }

    pub fn add_top_margin(&mut self, height: u32) {
        //TODO maybe add debug line for margin
        self.bytes
            .splice(0..0, vec![self.paper_color; (self.width * height) as usize]);
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

        let left_bytes = vec![self.paper_color; left as usize];
        let right_bytes = vec![self.paper_color; right as usize];

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

    pub fn consume_rgb_u8(&mut self) -> (u32, u32, Vec<u8>) {
        let w = self.width;
        let h = self.get_height();

        let mut pixels = Vec::with_capacity((w * h) as usize * 3);

        for byte in self.bytes.iter() {
            pixels.push(byte.r);
            pixels.push(byte.g);
            pixels.push(byte.b);
        }

        self.set_width(0);

        (w, h, pixels)
    }

    pub fn copy(&mut self) -> (u32, u32, Vec<RGBA>) {
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
