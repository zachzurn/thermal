use crate::context::{Context, Font, TextJustify, TextStrikethrough, TextUnderline};
use std::fmt;
use textwrap::WordSeparator;

#[derive(Clone)]
pub struct TextSpan {
    pub font: Font,
    pub character_width: u32,
    pub character_height: u32,
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
}

#[derive(Clone)]
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
            character_width: context.text.character_width as u32 * style.width_mult as u32,
            character_height: context.text.character_height as u32 * style.height_mult as u32,
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

    pub fn get_dimensions(&mut self, context: &Context) {
        self.dimensions = Some(Dimensions {
            x: context.get_x(),
            y: context.get_y(),
            w: self.text.len() as u32 * self.character_width,
            h: self.character_height,
        });
    }

    pub fn get_width(&self) -> u32 {
        self.text.len() as u32 * self.character_width
    }

    pub fn break_apart(&mut self, first_at: usize, rest_at: usize) -> Vec<TextSpan> {
        let text = self.text.clone();
        let first = &text[0..first_at];
        let last = &text[rest_at..];
        self.text = first.to_string();

        let spans = vec![];

        //break last by rest_at

        spans
    }

    pub fn break_into_words(&self) -> Vec<TextSpan> {
        let words = WordSeparator::UnicodeBreakProperties.find_words(self.text.as_str());

        words
            .map(|word| {
                let mut w = self.clone();
                w.text = format!("{}{}", word.word, word.whitespace);
                w
            })
            .collect()
    }
}

impl fmt::Debug for TextSpan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Span")
            .field("text", &self.text.replace("\n", "{LF}"))
            .finish()
    }
}
