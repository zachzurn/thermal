extern crate fontdue;
extern crate png;
extern crate textwrap;

use std::fs::File;
use std::io::BufWriter;
use std::mem::take;
use std::path::Path;
use std::rc::Rc;

use fontdue::layout::CharacterData;
use fontdue::Font;
use png::BitDepth;
use textwrap::WordSeparator;

use thermal_parser::context::{Context, PrintDirection, TextJustify, TextStrikethrough, TextUnderline};

const THRESHOLD: u8 = 120;
const SCALE_THRESHOLD: u8 = 140;

pub struct FontFamily {
    pub regular: fontdue::Font,
    pub bold: fontdue::Font,
    pub italic: fontdue::Font,
    pub bold_italic: fontdue::Font,
}

pub struct TextSpan {
    pub font: Rc<FontFamily>,
    pub size: u32,
    pub text: String,
    pub bold: bool,
    pub italic: bool,
    pub underline: usize,
    pub strikethrough: usize,
    pub stretch_width: f32,
    pub stretch_height: f32,
    pub inverted: bool,
    pub upside_down: bool,
    pub justify: TextJustify,
}

impl TextSpan {
    pub fn new(font: Rc<FontFamily>, text: String, context: &Context) -> Self {
        let style = &context.text;

        let underline = match style.underline {
            TextUnderline::On => context.points_to_pixels(1.0) as usize,
            TextUnderline::Double => context.points_to_pixels(2.0) as usize,
            _ => 0,
        };

        let strikethrough = match style.strikethrough {
            TextStrikethrough::On => context.points_to_pixels(1.0) as usize,
            TextStrikethrough::Double => context.points_to_pixels(2.0) as usize,
            _ => 0,
        };

        Self {
            font,
            size: context.font_size_pixels(),
            text,
            bold: style.bold,
            italic: style.italic,
            underline,
            strikethrough,
            stretch_width: style.width_mult as f32,
            stretch_height: style.height_mult as f32,
            inverted: style.invert,
            upside_down: style.upside_down,
            justify: context.text.justify.clone(),
        }
    }

    pub fn char_width(&self) -> usize {
        let metrics = self.font.regular.metrics(' ', self.size as f32);
        metrics.advance_width.floor() as usize * self.stretch_width as usize
    }

    pub fn get_font(&self) -> &Font {
        if self.bold && self.italic {
            return &self.font.bold_italic;
        }
        if self.bold {
            return &self.font.bold;
        }
        if self.italic {
            return &self.font.italic;
        }
        &self.font.regular
    }
}

pub struct TextLayout {
    pub spans: Vec<TextSpan>,
    pub line_height: usize,
    pub tab_len: usize,
}

/// A simple image renderer designed for thermal image generation
/// This allows for an image with a fixed width that can grow in height
/// to accommodate sets of pixels being pushed at arbitrary x and y values
pub struct ThermalImage {
    bytes: Vec<u8>,
    print_direction: PrintDirection,
    pub width: usize,
    pub font: Rc<FontFamily>,
}

impl ThermalImage {
    pub fn new(font: Rc<FontFamily>, width: usize) -> Self {
        Self {
            bytes: Vec::<u8>::new(),
            font,
            width,
            print_direction: PrintDirection::Left2Right
        }
    }

    //Print direction is a weird one that we emulate
    // by rotating the image
    pub fn set_print_direction(&mut self, direction: PrintDirection) {
        let current_rotation = match self.print_direction {
            PrintDirection::Left2Right => 0,
            PrintDirection::Bottom2Top => 0,
            PrintDirection::Right2Left => 0,
            PrintDirection::Top2Bottom => 0
        };
    }

    //Setting the width clears any bytes
    pub fn set_width(&mut self, width: usize) {
        self.width = width;
        self.bytes = Vec::<u8>::new();
    }

    pub fn reset(&mut self) {
        self.bytes.clear();
        self.bytes.shrink_to(0);
    }

    pub fn draw_rect(&mut self, x: usize, y: usize, w: usize, h: usize) {
        self.put_pixels(x, y, w, h, vec![0u8; w * h], false, true);
    }

    pub fn draw_text(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        layout: &mut TextLayout,
    ) -> (usize, usize) {
        let mut temp_x = 0;
        let newline = Vec::<(&TextSpan, String, usize)>::new();
        let mut lines = vec![newline.clone()];

        for span in &mut layout.spans {
            let char_width = span.char_width();
            let words = WordSeparator::UnicodeBreakProperties.find_words(span.text.as_str());

            for word in words {
                if word.word.contains('\t') {
                    let mut tab_len = layout.tab_len * span.char_width();
                    while tab_len < temp_x {
                        tab_len += tab_len;
                    }
                    if tab_len < width {
                        temp_x = tab_len
                    }
                    continue;
                }

                if word.word.contains('\r') {
                    temp_x = 0;
                    continue;
                }

                if word.word.contains('\n') {
                    lines.push(newline.clone());
                    temp_x = 0;
                    continue;
                }

                let word_len = word.word.len() + word.whitespace.len();
                if word_len * char_width < width - temp_x {
                    lines.last_mut().unwrap().push((
                        span,
                        format!("{}{}", word.word, word.whitespace),
                        temp_x,
                    ));
                    temp_x += word_len * char_width;
                } else if word_len * char_width > width {
                    let broken = word.break_apart(width / char_width);

                    for broke in broken {
                        let broke_word_len =
                            broke.word.len() as f32 + broke.whitespace.len() as f32;
                        if width as f32 - (broke_word_len * char_width as f32) < char_width as f32 {
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
                            temp_x += broke_word_len as usize * char_width;
                        }
                    }
                } else {
                    //New line and then add word
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

        let mut new_x = x;
        let mut new_y = y;

        for line in lines.into_iter() {
            let mut line_height_mult = 1;
            let mut precalculated_width = 0;
            let mut justify = TextJustify::Left;
            let mut iter = 0;

            for word in &line {
                if iter == 0 {
                    justify = word.0.justify.clone();
                }
                precalculated_width += word.1.len() * word.0.char_width();
                iter += 1;
            }

            //Prevent issues with line widths that are way too long
            if precalculated_width > width {
                println!(
                    "Precalc width too wide {} is less than {}",
                    width, precalculated_width
                );
                precalculated_width = width;
            }

            match justify {
                TextJustify::Center => new_x = (width - precalculated_width) / 2,
                TextJustify::Right => new_x = width - precalculated_width,
                _ => {}
            }

            for word in &line {
                if word.0.stretch_height > 1.0 {
                    line_height_mult = word.0.stretch_height as usize;
                }
                let (w, _) = self.render_word(new_x, new_y, word.1.as_str(), word.0);
                new_x += w;
            }
            new_x = x;
            new_y += layout.line_height as usize * line_height_mult;
        }

        (new_x, new_y)
    }

    pub fn render_word(
        &mut self,
        x: usize,
        y: usize,
        text: &str,
        span: &TextSpan,
    ) -> (usize, usize) {
        let font = span.get_font();
        let font_size = span.size as f32;
        let font_metrics = font.horizontal_line_metrics(font_size).unwrap();
        let mut w = 0;
        let mut h = (font_metrics.ascent + font_metrics.descent.abs()).ceil() as usize;
        let mut cur_x = x;

        let baseline = f32::ceil(font_metrics.ascent + font_metrics.descent);

        //Need a solution for graphemes maybe
        for char in text.chars() {
            let (metrics, bitmap) = font.rasterize(char, span.size as f32);

            let mut bitmap = bitmap;
            bitmap = self.scale_bitmap(
                &bitmap,
                metrics.width,
                metrics.height,
                span.stretch_width as usize,
                span.stretch_height as usize,
            );

            let glyph_index = font.lookup_glyph_index(char);
            let char_data = CharacterData::classify(char, glyph_index);

            if char_data.is_control() {
                continue;
            }

            if char_data.is_missing() {
                cur_x += span.char_width();
                continue;
            }

            let y_offset =
                f32::ceil((baseline - metrics.bounds.height) + (-1.0 * metrics.bounds.ymin))
                    as usize;
            let x_offset = cur_x + metrics.bounds.xmin.round().abs() as usize;

            self.put_pixels(
                x_offset,
                y + y_offset * span.stretch_height as usize,
                metrics.width * span.stretch_width as usize,
                metrics.height * span.stretch_height as usize,
                bitmap,
                true,
                true,
            );
            cur_x += span.char_width();
            w += span.char_width();
            h = h.max((metrics.height * span.stretch_height as usize) + y_offset);
        }

        if span.underline > 0 {
            let under_y = (y
                + (font_metrics.ascent * span.stretch_height) as usize
                + span.underline) as usize;
            let under_x = x;

            self.draw_rect(under_x, under_y, w, span.underline);
        }

        if span.strikethrough > 0 {
            let strike_y = y + ((font_metrics.ascent * span.stretch_height) / 2.0) as usize;
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
        width: usize,
        height: usize,
        stretch_width: usize,
        stretch_height: usize,
    ) -> Vec<u8> {
        let sw = width * stretch_width;
        let sh = height * stretch_height;

        let mut scaled = Vec::with_capacity(sw * sh);

        for y in 0..height {
            for _ in 0..stretch_height {
                for x in 0..width {
                    for _ in 0..stretch_width {
                        let pixel = if bitmap[width * y + x] < SCALE_THRESHOLD {
                            0
                        } else {
                            255
                        };
                        scaled.push(pixel)
                    }
                }
            }
        }

        scaled
    }

    pub fn invert_pixels(&mut self, x: usize, y: usize, width: usize, height: usize) {
        if x + width > self.width {
            return;
        };
        self.ensure_height(y + height);

        let mut cur_y = y;
        let mut cur_x = x;

        for _ in y..y + height {
            let idx = cur_y * self.width + cur_x;
            for i in 0..width {
                self.bytes[idx + i] = 255 - self.bytes[idx + i];
                cur_x += 1;
            }
            cur_x = x;
            cur_y += 1;
        }
    }

    pub fn flip_pixels(&mut self, x: usize, y: usize, width: usize, height: usize) {
        if x + width > self.width {
            return;
        };
        self.ensure_height(y + height);

        let mut sub_image = Vec::<u8>::with_capacity(width * height);

        for cur_y in y..y + height {
            let idx = cur_y * self.width + x;
            for cur_x in x..width {
                sub_image.push(self.bytes[idx + cur_x]);
            }
        }

        sub_image.reverse();

        self.put_pixels(x, y, width, height, sub_image, false, false);
    }

    pub fn put_pixels(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        pixels: Vec<u8>,
        invert: bool,
        multiply: bool,
    ) -> bool {
        let mut cur_x = x;
        let mut cur_y = y;

        if x + width > self.width {
            return false;
        };

        if pixels.len() < width * height {
            return false;
        };

        self.ensure_height(y + height);

        if multiply {
            for pixel in pixels {
                let idx = cur_y * self.width + cur_x;

                //ensure black or white only
                let pixel = if pixel < THRESHOLD { 0 } else { 255 };

                self.bytes[idx] = if invert {
                    u8::min(255 - pixel, self.bytes[idx])
                } else {
                    u8::min(pixel, self.bytes[idx])
                };
                if cur_x == x + width - 1 {
                    cur_x = x;
                    cur_y += 1;
                } else {
                    cur_x += 1;
                }
            }
        } else {
            for pixel in pixels {
                let idx = cur_y * self.width + cur_x;
                self.bytes[idx] = if invert { 255 - pixel } else { pixel };
                if cur_x == x + width - 1 {
                    cur_x = x;
                    cur_y += 1;
                } else {
                    cur_x += 1;
                }
            }
        }

        true
    }

    pub fn ensure_height(&mut self, height: usize) {
        let len = self.width * height;
        let cur_len = self.bytes.len();
        if cur_len >= len {
            return;
        }
        let to_add = len - cur_len;

        for _ in 0..to_add {
            self.bytes.push(255u8);
        }
    }

    pub fn add_top_margin(&mut self, height: usize) {
        self.bytes.splice(0..0, vec![255u8; self.width * height]);
    }

    pub fn expand_to_width(&mut self, new_width: usize) {
        let old_width = self.width;
        if new_width < old_width {
            return;
        };
        let height = self.bytes.len() / self.width;
        let left = ((new_width - old_width) as f32 / 2.0).floor() as usize;
        let right = new_width - (left + old_width);

        self.bytes
            .try_reserve(height * new_width - self.bytes.len())
            .unwrap();

        let mut insert_idx = 0;

        let left_bytes = vec![255u8; left];
        let right_bytes = vec![255u8; right];

        for _ in 0..height {
            self.bytes
                .splice(insert_idx..insert_idx, left_bytes.clone());
            insert_idx += left + old_width;

            self.bytes
                .splice(insert_idx..insert_idx, right_bytes.clone());
            insert_idx += right;
        }

        self.width = new_width;
    }

    pub fn copy(&mut self) -> (usize, usize, Vec<u8>) {
        let pixels = take(&mut self.bytes);
        let w = self.width;
        let h = pixels.len() / self.width;
        self.width = 0;
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
