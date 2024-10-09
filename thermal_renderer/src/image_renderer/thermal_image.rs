extern crate fontdue;
extern crate png;
extern crate textwrap;

use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::rc::Rc;

use fontdue::layout::CharacterData;
use png::BitDepth;
use thermal_parser::graphics::{Image, Rectangle};
use thermal_parser::text::TextSpan;

const THRESHOLD: u8 = 120;
const SCALE_THRESHOLD: u8 = 140;

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
        }
    }

    pub fn char_width(&self, span: &TextSpan) -> u32 {
        let metrics = self.font.regular.metrics(' ', span.size as f32);
        metrics.advance_width.floor() as u32 * span.stretch_width as u32
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

    pub fn draw_text(&mut self, mut layout: TextLayout) -> (u32, u32) {
        let mut temp_x = 0u32;
        let width = layout.max_w;
        let newline = Vec::<(&TextSpan, String, u32)>::new();
        let mut lines = vec![newline.clone()];
    
        for span in &mut layout.spans {
            let char_width = self.char_width(span);
            let words = WordSeparator::UnicodeBreakProperties.find_words(span.text.as_str());
    
            for word in words {
                let word_len: u32 = (word.word.len() + word.whitespace.len()) as u32;
                if word_len * char_width <= width - temp_x {
                    //Word fits on current line
                    lines.last_mut().unwrap().push((
                        span,
                        format!("{}{}", word.word, word.whitespace),
                        temp_x,
                    ));
                    temp_x += word_len * char_width;
                } else if word_len * char_width > width {
                    //Break the word into parts for super long words
                    let broken = word.break_apart((width / char_width) as usize);
    
                    for broke in broken {
                        let broke_word_len =
                            broke.word.len() as f32 + broke.whitespace.len() as f32;
                        if layout.max_w as f32 - (broke_word_len * char_width as f32)
                            < char_width as f32
                        {
                            lines.push(newline.clone());
                            temp_x = 0;
                            lines.last_mut().unwrap().push((
                                span,
                                format!("{}{}", broke.word, broke.whitespace),
                                temp_x,
                            ));
                            lines.push(newline.clone());
                        } else {
                            lines.last_mut().unwrap().push((
                                span,
                                format!("{}{}", broke.word, broke.whitespace),
                                temp_x,
                            ));
                            temp_x += broke_word_len as u32 * char_width;
                        }
                    }
                } else {
                    //Put word on a newline
                    lines.push(newline.clone());
                    temp_x = 0;
                    lines.last_mut().unwrap().push((
                        span,
                        format!("{}{}", word.word, word.whitespace),
                        temp_x,
                    ));
                    temp_x += word_len * char_width;
                }
            }
        }
    
        let mut new_x = layout.x;
        let mut new_y = layout.y;
    
        for line in lines.into_iter() {
            let mut line_height = layout.line_height;
            let mut precalculated_width = 0u32;
            let mut justify = TextJustify::Left;
            let mut iter = 0;
    
            for word in &line {
                if iter == 0 {
                    justify = word.0.justify.clone();
                }
                precalculated_width += word.1.len() as u32 * self.char_width(word.0);
                iter += 1;
            }
    
            //Prevent issues with line widths that are way too long
            //TODO write some tests for this
            if precalculated_width > width {
                precalculated_width = width;
            }
    
            match justify {
                TextJustify::Center => {
                    new_x = (width - precalculated_width) / 2;
                    println!("CENTER w{} x{} line w{}", width, new_x, precalculated_width);
                },
                TextJustify::Right => {
                    println!("RIGHT");
                    new_x = width - precalculated_width;
                }
                _ => {}
            }
    
            for word in &line {
                if word.1.contains('\t') {
                    let mut tab_len = layout.tab_len * self.char_width(word.0);
                    while tab_len < temp_x {
                        tab_len += tab_len;
                    }
                    if tab_len < layout.max_w {
                        new_x = tab_len;
                    }
                    continue;
                }
    
                if word.1.contains('\r') {
                    new_x = layout.base_x;
                    continue;
                }
    
                if word.1.eq("\n") {
                    new_y += layout.line_height;
                    new_x = layout.base_x;
                    continue;
                }
    
                let (w, h) = self.render_word(new_x, new_y, word.1.as_str(), word.0);
                new_x += w;
            }
        }
    
        (new_x as u32, new_y as u32)
    }

    pub fn render_word(&mut self, x: u32, y: u32, text: &str, span: &TextSpan) -> (u32, u32) {
        let font = self.get_font(span);
        let font_size = span.size as f32;
        let font_metrics = font.horizontal_line_metrics(font_size).unwrap();
        let mut w = 0;
        let mut h = (font_metrics.ascent + font_metrics.descent.abs()).ceil() as u32;
        let mut cur_x = x;

        let baseline = f32::ceil(font_metrics.ascent + font_metrics.descent);

        //Need a solution for graphemes maybe
        for char in text.chars() {
            let (metrics, bitmap) = font.rasterize(char, span.size as f32);

            let mut bitmap = bitmap;
            bitmap = self.scale_bitmap(
                &bitmap,
                metrics.width as u32,
                metrics.height as u32,
                span.stretch_width as u32,
                span.stretch_height as u32,
            );

            let glyph_index = font.lookup_glyph_index(char);
            let char_data = CharacterData::classify(char, glyph_index);

            if char_data.is_control() {
                continue;
            }

            if char_data.is_missing() {
                cur_x += self.char_width(span);
                continue;
            }

            let y_offset =
                f32::ceil((baseline - metrics.bounds.height) + (-1.0 * metrics.bounds.ymin)) as u32;
            let x_offset = cur_x + metrics.bounds.xmin.round().abs() as u32;

            self.put_pixels(
                x_offset,
                y + y_offset * span.stretch_height as u32,
                metrics.width as u32 * span.stretch_width as u32,
                metrics.height as u32 * span.stretch_height as u32,
                bitmap,
                true,
                true,
            );
            cur_x += self.char_width(span);
            w += self.char_width(span);
            h = h.max(metrics.height as u32 * span.stretch_height as u32 + y_offset);
        }

        if span.underline > 0 {
            let under_y =
                (y + (font_metrics.ascent * span.stretch_height) as u32 + span.underline) as u32;
            let under_x = x;

            self.draw_rect(under_x, under_y, w, span.underline);
        }

        if span.strikethrough > 0 {
            let strike_y = y + ((font_metrics.ascent * span.stretch_height) / 2.0) as u32;
            let strike_x = x;

            self.draw_rect(
                strike_x,
                strike_y - span.strikethrough,
                w,
                span.strikethrough,
            );
        }

        if span.inverted {
            self.invert_pixels(x, y, w, h);
        }

        if span.upside_down {
            self.flip_pixels(x, y, w, h);
        }

        (w, h)
    }

    pub fn scale_bitmap(
        &mut self,
        bitmap: &Vec<u8>,
        width: u32,
        height: u32,
        stretch_width: u32,
        stretch_height: u32,
    ) -> Vec<u8> {
        let sw = width * stretch_width;
        let sh = height * stretch_height;

        // Pre-allocate the correct size
        let mut scaled = vec![0u8; (sw * sh) as usize];

        for y in 0..height {
            for dy in 0..stretch_height {
                let scaled_y = y * stretch_height + dy;

                for x in 0..width {
                    for dx in 0..stretch_width {
                        let scaled_x = x * stretch_width + dx;

                        let pixel =
                            if bitmap[width as usize * y as usize + x as usize] < SCALE_THRESHOLD {
                                0
                            } else {
                                255
                            };

                        scaled[scaled_y as usize * sw as usize + scaled_x as usize] = pixel;
                    }
                }
            }
        }

        scaled
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

    pub fn save_png(&self, filepath: String) {
        if self.bytes.len() == 0 || self.width == 0 {
            println!("Nothing to save!");
            return;
        }
        let path = Path::new(&filepath);
        let file = File::create(path).unwrap();
        let ref mut w = BufWriter::new(file);
        let mut encoder = png::Encoder::new(
            w,
            self.width as u32,
            self.bytes.len() as u32 / self.width as u32,
        );
        encoder.set_color(png::ColorType::Grayscale);
        encoder.set_depth(BitDepth::Eight);

        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(&self.bytes).unwrap(); // Save
    }
}
