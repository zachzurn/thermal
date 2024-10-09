use std::fmt;
use crate::context::{Context, Font, TextJustify, TextStrikethrough, TextUnderline};

#[derive(Clone)]
pub struct TextSpan {
    pub font: Font,
    pub size: u32,
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
}

impl fmt::Debug for TextSpan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Span")
            .field("text", &self.text.replace("\n", "{LF}"))
            .finish()
    }
}

pub struct PositionedTextSpan {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
    pub span: TextSpan,
}