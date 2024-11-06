use crate::context::{Context, Font, TextJustify, TextStrikethrough, TextUnderline};
use crate::graphics::RGBA;
use std::fmt;

#[derive(Clone)]
pub struct TextSpan {
    pub font: Font,
    pub character_width: u32,
    pub character_height: u32,
    pub base_character_width: u32,
    pub base_character_height: u32,
    pub text: String,
    pub bold: bool,
    pub italic: bool,
    pub underline: u32,
    pub strikethrough: u32,
    pub stretch_width: f32,
    pub stretch_height: f32,
    pub inverted: bool,
    pub upside_down: bool,
    pub justify: TextJustify,
    pub dimensions: Option<Dimensions>,
    pub background_color: RGBA,
    pub text_color: RGBA,
}

#[derive(Clone, Debug)]
pub struct Dimensions {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

impl TextSpan {
    pub fn new(text: String, context: &Context) -> Self {
        let style = &context.text;

        let underline = match style.underline {
            TextUnderline::On => context.points_to_pixels(1.0) as u32,
            TextUnderline::Double => context.points_to_pixels(2.0) as u32,
            _ => 0,
        };

        let strikethrough = match style.strikethrough {
            TextStrikethrough::On => context.points_to_pixels(1.0) as u32,
            TextStrikethrough::Double => context.points_to_pixels(2.0) as u32,
            _ => 0,
        };

        Self {
            font: context.text.font.clone(),
            base_character_width: style.character_width as u32,
            base_character_height: style.character_height as u32,
            character_width: style.character_width as u32 * style.width_mult as u32,
            character_height: style.character_height as u32 * style.height_mult as u32,
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
            dimensions: None,
        }
    }

    pub fn new_for_barcode(text: String, context: &Context) -> Self {
        let mut span = TextSpan::new(text, context);
        span.font = context.barcode.font.clone();
        span
    }

    pub fn get_dimensions(&mut self, context: &Context) {
        self.dimensions = Some(Dimensions {
            x: context.get_x(),
            y: context.get_y(),
            w: self.get_width(),
            h: self.character_height,
        });
    }

    pub fn get_width(&self) -> u32 {
        self.character_count() * self.character_width
    }

    pub fn character_count(&self) -> u32 {
        self.text.chars().count() as u32
    }

    pub fn clone_with(&self, string: String) -> Self {
        let mut clone = TextSpan {
            font: self.font.clone(),
            character_width: self.character_width,
            character_height: self.character_height,
            base_character_width: self.base_character_width,
            base_character_height: self.base_character_height,
            text: "".to_string(),
            bold: self.bold,
            italic: self.italic,
            underline: self.underline,
            strikethrough: self.strikethrough,
            stretch_width: self.stretch_width,
            stretch_height: self.stretch_height,
            inverted: self.inverted,
            upside_down: self.upside_down,
            justify: self.justify.clone(),
            dimensions: None,
        };
        clone.text = string;
        clone
    }

    pub fn break_apart(&self, first_line_length: usize, line_length: usize) -> Vec<TextSpan> {
        if line_length == 0 {
            panic!("break_apart called with zero line length");
        }

        let chars: Vec<char> = self.text.chars().collect();
        let mut result = Vec::new();
        let mut index = 0;

        // First split, which is often smaller
        if first_line_length > 0 && chars.len() > 0 {
            let first_chunk = chars
                .iter()
                .take(first_line_length)
                .cloned()
                .collect::<String>();
            result.push(self.clone_with(first_chunk));
            index += first_line_length;
        }

        //We are always expecting a first line value, even if there isn't one
        if index == 0 {
            result.push(self.clone_with("".to_string()));
        }

        // Split the rest of the string into chunks
        while index < chars.len() {
            let chunk = chars
                .iter()
                .skip(index)
                .take(line_length)
                .cloned()
                .collect::<String>();
            result.push(self.clone_with(chunk));
            index += line_length;
        }

        result
    }

    pub fn break_into_words(&self) -> Vec<TextSpan> {
        let mut words = Vec::new();
        let mut current_word = String::new();

        for c in self.text.chars() {
            if c.is_whitespace() {
                // If we encounter a whitespace character, we add it to the current word
                current_word.push(c);

                if !current_word.is_empty() {
                    words.push(self.clone_with(current_word.clone()));
                    current_word.clear();
                }
            } else {
                // Add regular characters to the current word
                current_word.push(c);
            }
        }

        // Push the last word if there is any
        if !current_word.is_empty() {
            words.push(self.clone_with(current_word));
        }

        words
    }
}

impl fmt::Debug for TextSpan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Span")
            .field("text", &self.text.replace("\n", "{LF}"))
            .field("dim", &self.dimensions.clone())
            .finish()
    }
}
