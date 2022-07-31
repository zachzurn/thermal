extern crate fontdue;
extern crate textwrap;
extern crate png;

use std::path::Path;
use std::fs::File;
use std::io::BufWriter;

use std::rc::Rc;
use fontdue::Font;
use fontdue::layout::{CharacterData, GlyphRasterConfig};
use png::BitDepth;
use textwrap::{WordSeparator, core::Word};
use textwrap::core::Fragment;

use thermal_parser::command::DeviceCommand;
use thermal_parser::context::{Context, TextContext};

pub struct FontFamily {
    pub regular: fontdue::Font,
    pub bold: fontdue::Font,
    pub italic: fontdue::Font,
    pub bold_italic: fontdue::Font
}

pub struct TextSpan {
    pub font: Rc<FontFamily>,
    pub size: u32,
    pub text: String,
    pub bold: bool,
    pub italic: bool,
    pub underline: thermal_parser::context::TextUnderline,
    pub strikethrough: thermal_parser::context::TextStrikethrough,
    pub stretch_width: f32,
    pub stretch_height: f32,
    pub inverted: bool,
    c_width: usize,
}

impl TextSpan {
    pub fn new(font: Rc<FontFamily>, text: String, context: &Context) -> Self{
        let style = &context.text;

        Self {
            font,
            size: context.font_size_pixels(),
            text,
            bold: style.bold,
            italic: style.italic,
            underline: style.underline.clone(),
            strikethrough: style.strikethrough.clone(),
            stretch_width: style.width_mult as f32,
            stretch_height: style.height_mult as f32,
            inverted: style.invert,
            c_width: 0
        }
    }

    pub fn char_width(&self) -> usize {
        let metrics = self.font.regular.metrics(' ', self.size as f32);
        (metrics.advance_width * self.stretch_width) as usize
    }

    pub fn get_font(&self) -> &Font {
        if self.bold && self.italic { return &self.font.bold_italic }
        if self.bold { return &self.font.bold }
        if self.italic { return &self.font.italic }
        &self.font.regular
    }
}

pub struct TextLayout {
    pub spans: Vec<TextSpan>,
    pub line_height: usize,
    pub justify: thermal_parser::context::TextJustify
}

pub struct ThermalImage {
    bytes: Vec<u8>,
    pub width: usize,
    pub font: Rc<FontFamily>
}

impl ThermalImage {
    pub fn new(font: Rc<FontFamily>, width: usize) -> Self {
        Self {
            bytes: Vec::<u8>::new(),
            font,
            width,
        }
    }

    //Setting the width clears any bytes
    pub fn set_width(&mut self, width: usize) {
        self.width = width;
        self.bytes = Vec::<u8>::new();
    }

    pub fn reset(&mut self){
        self.bytes.clear();
        self.bytes.shrink_to(0);
    }

    pub fn draw_rect(&mut self, x: usize, y: usize, w: usize, h: usize){
        self.put_pixels(x, y, w, h, vec![0u8; w * h], false);
    }

    pub fn draw_text(&mut self, x: usize, y: usize, width: usize, layout: &mut TextLayout) -> (usize, usize) {
        let mut temp_x = 0;
        let mut newline = Vec::<(&TextSpan, String)>::new();
        let mut lines = vec![newline.clone()];

        for span in &mut layout.spans {
            let char_width = span.char_width();
            let words = WordSeparator::UnicodeBreakProperties.find_words(span.text.as_str());

            for word in words {
                if word.word.contains('\n'){
                    lines.push(newline.clone());
                    continue;
                }

                let word_len = word.word.len() + word.whitespace.len();
                if word_len * char_width < width - temp_x {
                    lines.last_mut().unwrap().push((span, format!("{}{}", word.word, word.whitespace)));
                    temp_x += word_len * char_width;
                } else if word_len * char_width > width {
                    //break the word up based on available chars
                    let broken = word.break_apart(width / char_width);

                    for broke in broken {
                        let broke_word_len = broke.word.len() as f32 + broke.whitespace.len() as f32;
                        if width as f32 - (broke_word_len * char_width as f32) < char_width as f32 {
                            lines.push(newline.clone());
                            temp_x = 0;
                            lines.last_mut().unwrap().push((span, format!("{}{}", broke.word, broke.whitespace)));
                            lines.push(newline.clone());
                        } else {
                            lines.last_mut().unwrap().push((span, format!("{}{}", broke.word, broke.whitespace)));
                            temp_x += broke_word_len as usize * char_width;
                        }
                    }
                } else {
                    //New line and then add word
                    lines.push(newline.clone());
                    temp_x = 0;
                    lines.last_mut().unwrap().push((span, format!("{}{}", word.word, word.whitespace)));
                    temp_x += word_len * char_width;
                }
            }
        }

        //Render the lines
        let mut new_x = x;
        let mut new_y = y;

        for line in &mut lines {
            for item in line {
                let (w,h) = self.render_word(new_x, new_y, item.1.as_str(), item.0);
                //TODO draw underline and strike through
                new_x += w;
            }
            new_x = x;
            new_y += layout.line_height as usize
        }

        (new_x, new_y)
    }

    pub fn render_word(&mut self, x: usize, y: usize, text: &str, span: &TextSpan) -> (usize, usize){
        let font = span.get_font();
        let font_size = span.size as f32;
        let font_metrics = font.horizontal_line_metrics(font_size).unwrap();
        let mut w = 0;
        let h = f32::ceil(font_metrics.ascent + font_metrics.descent) as usize;
        let mut cur_x = x;
        let mut cur_y = y;

        let baseline = font_metrics.ascent + font_metrics.descent;

        //Need a solution for graphemes maybe
        for char in text.chars() {
            let (metrics, mut bitmap) = font.rasterize(char, span.size as f32);

            let glyph_index = font.lookup_glyph_index(char);
            let char_data = CharacterData::classify(char, glyph_index);

            if char_data.is_control() {
                continue;
            }

            if char_data.is_missing() {
                cur_x += span.char_width();
                continue;
            }

            //This may need some adjustment, seems like at small sizes the letters don't line up properly (specifically the i)
            let y_offset = f32::ceil((baseline - metrics.bounds.height) + (-1.0 * metrics.bounds.ymin)) as usize;
            let x_offset = cur_x + metrics.bounds.xmin.round().abs() as usize;

            self.put_pixels(x_offset, cur_y + y_offset, metrics.width, metrics.height, bitmap, true);
            cur_x += span.char_width();
            w += span.char_width();
        }

        (w,h)
    }

    pub fn put_pixels(&mut self, x: usize, y: usize, width: usize, height: usize, pixels: Vec<u8>, invert: bool) -> bool{
        let mut cur_x = x;
        let mut cur_y = y;

        if x + width > self.width {
            println!("TOO BIG, IGNORING PIXELS {} vs {} {}", self.width, x, width);
            return false
        };

        if pixels.len() < width * height {
            println!("IMAGE BYTE COUNT IS WRONG len {}  w:{} h:{} exp:{}", pixels.len(), width, height, width * height);
            return false
        };

        self.ensure_height(y + height);

        for pixel in pixels {
            let idx = cur_y * self.width + cur_x;
            //we use a darken here to simulate thermal printing which is additive
            self.bytes[idx] = if invert { u8::min(255 - pixel, self.bytes[idx]) } else { u8::min(pixel, self.bytes[idx]) };
            if cur_x == x + width - 1 { cur_x = x; cur_y+=1; } else { cur_x += 1; }
        }
        true
    }

    pub fn ensure_height(&mut self, height: usize){
        let len = self.width * height;
        let cur_len = self.bytes.len();
        if cur_len >= len { return }
        let to_add = len - cur_len;

        for i in 0..to_add {
            self.bytes.push(255u8);
        }
    }

    pub fn expand(&mut self, left: usize, right: usize, top: usize, bottom: usize){

    }

    pub fn save_png(&self, filepath: String){
        if self.bytes.len() == 0 || self.width == 0 {
            println!("Nothing to save!");
            return;
        }
        let path = Path::new(&filepath);
        let file = File::create(path).unwrap();
        let ref mut w = BufWriter::new(file);
        let mut encoder = png::Encoder::new(w, self.width as u32, self.bytes.len() as u32 / self.width as u32);
        encoder.set_color(png::ColorType::Grayscale);
        encoder.set_depth(BitDepth::Eight);

        let mut writer = encoder.write_header().unwrap();

        writer.write_image_data(&self.bytes).unwrap(); // Save
    }

}
