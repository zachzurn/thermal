use crate::graphics;
use std::collections::HashMap;

use crate::graphics::{Image, ImageRef};

#[derive(Clone, PartialEq)]
pub enum TextJustify {
    Left,
    Center,
    Right,
}

#[derive(Clone, PartialEq)]
pub enum TextStrikethrough {
    Off,
    On,
    Double,
}

#[derive(Clone, PartialEq)]
pub enum TextUnderline {
    Off,
    On,
    Double,
}

#[derive(Clone, PartialEq)]
pub enum Font {
    A,
    B,
    C,
    D,
    E,
    SpecialA,
    SpecialB,
}

impl Font {
    pub fn from_raw(byte: u8) -> Font {
        match byte {
            0 | 48 => Font::A,
            1 | 49 => Font::B,
            2 | 50 => Font::C,
            3 | 51 => Font::D,
            4 | 52 => Font::E,
            97 => Font::SpecialA,
            98 => Font::SpecialB,
            _ => Font::A,
        }
    }
}

#[derive(Clone)]
pub enum HumanReadableInterface {
    None,
    Above,
    Below,
    Both,
}

#[derive(Clone)]
pub enum Color {
    Black,
    Red,
}

#[derive(Clone)]
pub struct Context {
    pub default: Option<Box<Context>>,
    pub text: TextContext,
    pub barcode: BarcodeContext,
    pub code2d: Code2DContext,
    pub graphics: GraphicsContext,
    pub is_page_mode: bool,
}

#[derive(Clone)]
pub struct TextContext {
    pub character_set: u8,
    pub code_table: u8,
    pub font_size: u8,
    pub justify: TextJustify,
    pub font: Font,
    pub bold: bool,
    pub italic: bool,
    pub underline: TextUnderline,
    pub strikethrough: TextStrikethrough,
    pub invert: bool,
    pub width_mult: u8,
    pub height_mult: u8,
    pub upside_down: bool,
    pub line_spacing: u8,
    pub color: Color,
    pub smoothing: bool,
    pub tab_len: u8, //character width for tabs
}

#[derive(Clone)]
pub struct GraphicsContext {
    pub x: usize,
    pub y: usize,
    pub paper_width: f32,
    pub margin_left: f32,
    pub margin_right: f32,
    pub dots_per_inch: u16,
    pub v_motion_unit: u8,
    pub h_motion_unit: u8,
    pub graphics_count: u16,
    pub stored_graphics: HashMap<ImageRef, Image>,
    pub buffer_graphics: Option<Image>,
}

#[derive(Clone)]
pub struct BarcodeContext {
    pub human_readable: HumanReadableInterface,
    pub width: u8,
    pub height: u8,
    pub font: Font,
}

#[derive(Clone)]
pub struct Code2DContext {
    pub symbol_storage: Option<graphics::Code2D>,

    pub qr_model: u8,
    pub qr_size: u8,
    pub qr_err_correction: u8,

    pub pdf417_columns: u8,
    pub pdf417_rows: u8,
    pub pdf417_width: u8,
    pub pdf417_row_height: u8,
    pub pdf417_err_correction: u8,
    pub pdf417_is_truncated: bool,

    pub maxicode_mode: u8,

    pub gs1_databar_width: u8,
    pub gs1_databar_max_width: u32,

    pub composite_width: u8,
    pub composite_max_width: u32,
    pub composite_font: Font,

    pub aztec_mode: u8,
    pub aztec_layers: u8,
    pub aztec_size: u8,
    pub aztec_error_correction: u8,

    pub datamatrix_type: u8,
    pub datamatrix_columns: u8,
    pub datamatrix_rows: u8,
    pub datamatrix_width: u8,
}

impl Context {
    fn default() -> Context {
        Context {
            default: None,
            text: TextContext {
                character_set: 0,
                code_table: 0,
                font_size: 10,
                justify: TextJustify::Left,
                font: Font::A,
                bold: false,
                italic: false,
                underline: TextUnderline::Off,
                strikethrough: TextStrikethrough::Off,
                invert: false,
                width_mult: 1,
                height_mult: 1,
                upside_down: false,
                line_spacing: 30, //pixels
                color: Color::Black,
                smoothing: false,
                tab_len: 10,
            },
            barcode: BarcodeContext {
                human_readable: HumanReadableInterface::None,
                width: 2,
                height: 40,
                font: Font::A,
            },
            code2d: Code2DContext {
                symbol_storage: None,
                qr_model: 0,
                qr_size: 0,
                qr_err_correction: 0,
                pdf417_columns: 0,
                pdf417_rows: 0,
                pdf417_width: 0,
                pdf417_row_height: 0,
                pdf417_err_correction: 0,
                pdf417_is_truncated: false,
                maxicode_mode: 0,
                gs1_databar_width: 0,
                gs1_databar_max_width: 0,
                composite_width: 0,
                composite_max_width: 0,
                composite_font: Font::A,
                aztec_mode: 0,
                aztec_layers: 0,
                aztec_size: 0,
                aztec_error_correction: 0,
                datamatrix_type: 0,
                datamatrix_columns: 0,
                datamatrix_rows: 0,
                datamatrix_width: 0,
            },
            graphics: GraphicsContext {
                x: 0,
                y: 0,
                paper_width: 3.0,   //inches
                margin_left: 0.1,   //inches
                margin_right: 0.1,  //inches
                dots_per_inch: 210, //pixels
                v_motion_unit: 1,   //Pixels
                h_motion_unit: 1,   //Pixels
                graphics_count: 0,
                stored_graphics: HashMap::<ImageRef, Image>::new(),
                buffer_graphics: None,
            },
            is_page_mode: false,
        }
    }

    pub fn new() -> Context {
        let default_context = Context::default();
        let mut new_context = default_context.clone();
        new_context.default = Some(Box::from(default_context));
        new_context
    }

    pub fn reset(&mut self) {
        if let Some(default) = &self.default {
            self.text = default.text.clone();
            self.barcode = default.barcode.clone();
            self.code2d = default.code2d.clone();
            self.graphics = default.graphics.clone();
        }
    }

    pub fn available_width_pixels(&self) -> u32 {
        let print_area =
            self.graphics.paper_width - (self.graphics.margin_left + self.graphics.margin_right);
        let print_area_pixels = print_area * self.graphics.dots_per_inch as f32;
        print_area_pixels.round() as u32
    }

    pub fn font_size_pixels(&self) -> u32 {
        //1 point = 72 pixels
        let pixels_per_point = self.graphics.dots_per_inch as f32 / 96f32;
        (self.text.font_size as f32 * pixels_per_point) as u32
    }

    pub fn points_to_pixels(&self, points: f32) -> u32 {
        let pixels_per_point = self.graphics.dots_per_inch as f32 / 96f32;
        (points * pixels_per_point) as u32
    }

    pub fn graphics_x_offset(&self, width: u32) -> u32 {
        if width > self.available_width_pixels() {
            return 0;
        }
        match self.text.justify {
            TextJustify::Center => {
                let center_remaining = self.available_width_pixels() - width;
                if center_remaining > 0 {
                    (center_remaining / 2) as u32
                } else {
                    0
                }
            }
            TextJustify::Right => self.available_width_pixels() - width,
            _ => 0,
        }
    }

    pub fn motion_unit_y_pixels(&self) -> u32 {
        self.graphics.v_motion_unit as u32
    }

    pub fn motion_unit_x_pixels(&self) -> u32 {
        self.graphics.h_motion_unit as u32
    }

    pub fn line_height_pixels(&self) -> u32 {
        self.text.line_spacing as u32 * self.motion_unit_y_pixels() as u32
    }
}
