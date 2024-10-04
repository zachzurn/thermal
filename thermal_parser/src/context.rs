use crate::decoder::{get_codepage, Codepage};
use crate::graphics;
use std::collections::HashMap;
use std::mem;

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
    pub page_mode: PageModeContext,
}

#[derive(Clone)]
pub struct TextContext {
    pub character_set: u8,
    pub code_table: u8,
    pub decoder: Codepage,
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

#[derive(Clone, Debug)]
pub enum PrintDirection {
    TopLeft2Right,
    BottomRight2Left,
    TopRight2Bottom,
    BottomLeft2Top,
}

#[derive(Clone)]
pub struct PageArea {
    pub x: usize,
    pub y: usize,
    pub w: usize,
    pub h: usize,
}

#[derive(Clone)]
pub struct PageModeContext {
    //Is page mode enabled
    pub enabled: bool,

    //Raw renderable area
    pub logical_area: PageArea,

    //Actual graphics context renderable area
    //Generally a translated version of the logical
    //area
    pub render_area: PageArea,

    //Total page area, can grow when render area
    //is changed
    pub page_area: PageArea,

    //Page mode print direction
    pub direction: PrintDirection,
    pub previous_direction: PrintDirection,
}

pub enum Rotation {
    R0,
    R90,
    R180,
    R270,
}

impl PageModeContext {
    pub fn apply_logical_area(&mut self) -> (Rotation, usize, usize) {
        let rotation =
            self.calculate_directional_rotation(&self.previous_direction, &self.direction);

        //Swap page area w and h
        let previously_swapped = PageModeContext::should_dimension_swap(&self.previous_direction);
        let should_swap = PageModeContext::should_dimension_swap(&self.direction);

        //Swap page dimension
        if !previously_swapped && should_swap || !should_swap && previously_swapped {
            mem::swap(&mut self.page_area.w, &mut self.page_area.h);
        }

        //Translate logical area to render area
        match self.direction {
            PrintDirection::TopLeft2Right => self.translate_top_left_to_right(),
            PrintDirection::BottomRight2Left => self.translate_bottom_right_to_left(),
            PrintDirection::TopRight2Bottom => self.translate_top_right_to_bottom(),
            PrintDirection::BottomLeft2Top => self.translate_bottom_left_to_top(),
        };

        //Set base values for x and y, render area will use these when resetting to y=0
        self.page_area.x = self.render_area.x;
        self.page_area.y = self.render_area.y;

        let render_max_width = self.render_area.x + self.render_area.w;
        let render_max_height = self.render_area.y + self.render_area.h;

        self.page_area.w = render_max_width.max(self.page_area.w);
        self.page_area.h = render_max_height.max(self.page_area.h);

        (rotation, self.page_area.w, self.page_area.h)
    }

    fn should_dimension_swap(direction: &PrintDirection) -> bool {
        match direction {
            PrintDirection::TopLeft2Right | PrintDirection::BottomRight2Left => false,
            _ => true,
        }
    }

    fn translate_top_left_to_right(&mut self) {
        let l = &self.logical_area;
        let r = &mut self.render_area;

        r.w = l.w;
        r.h = l.h;
        r.x = l.x;
        r.y = l.y;
    }

    fn translate_bottom_right_to_left(&mut self) {
        let l = &self.logical_area;
        let r = &mut self.render_area;
        let p = &mut self.page_area;

        r.w = l.w;
        r.h = l.h;
        r.y = l.y;
        r.x = p.w - (l.x + l.w);
    }

    fn translate_top_right_to_bottom(&mut self) {
        let l = &self.logical_area;
        let r = &mut self.render_area;
        let p = &mut self.page_area;

        r.w = l.h;
        r.h = l.w;
        r.x = p.w - (l.y + l.h);
        r.y = p.h - (l.x + l.w);
    }

    fn translate_bottom_left_to_top(&mut self) {
        let l = &self.logical_area;
        let r = &mut self.render_area;
        let p = &mut self.page_area;

        r.w = l.h;
        r.h = l.w;
        r.x = p.w - (l.y + l.h);
        r.y = l.x;

        // println!(
        //     "width{} minus lx{} + lw{} ({}) which is {}",
        //     p.w,
        //     l.y,
        //     l.h,
        //     l.y + l.h,
        //     p.w - (l.y + l.h)
        // );
        //
        // println!("Y should be {}", l.x);
    }

    pub fn calculate_directional_rotation(
        &self,
        from: &PrintDirection,
        to: &PrintDirection,
    ) -> Rotation {
        let previous = match from {
            PrintDirection::TopRight2Bottom => 3,
            PrintDirection::BottomRight2Left => 2,
            PrintDirection::BottomLeft2Top => 1,
            PrintDirection::TopLeft2Right => 0,
        };

        let current = match to {
            PrintDirection::TopRight2Bottom => 3,
            PrintDirection::BottomRight2Left => 2,
            PrintDirection::BottomLeft2Top => 1,
            PrintDirection::TopLeft2Right => 0,
        };

        let orientation_delta = (current as i8 - previous as i8).rem_euclid(4) as u8;

        //Come up with the rotation change that will
        //put page mode render area into the correct
        //render orientation
        match orientation_delta {
            1 => Rotation::R90,
            2 => Rotation::R180,
            3 => Rotation::R270,
            _ => Rotation::R0,
        }
    }
}

impl Context {
    fn default() -> Context {
        Context {
            default: None,
            text: TextContext {
                character_set: 0,
                code_table: 0,
                decoder: get_codepage(0, 0),
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
                dots_per_inch: 203, //pixels
                v_motion_unit: 1,   //Pixels
                h_motion_unit: 1,   //Pixels
                graphics_count: 0,
                stored_graphics: HashMap::<ImageRef, Image>::new(),
                buffer_graphics: None,
            },
            page_mode: PageModeContext {
                enabled: false,
                logical_area: PageArea {
                    x: 0,
                    y: 0,
                    w: 0,
                    h: 0,
                },
                render_area: PageArea {
                    x: 0,
                    y: 0,
                    w: 0,
                    h: 0,
                },
                page_area: PageArea {
                    x: 0,
                    y: 0,
                    w: 0,
                    h: 0,
                },
                direction: PrintDirection::TopLeft2Right,
                previous_direction: PrintDirection::TopLeft2Right,
            },
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

    pub fn update_decoder(&mut self) {
        self.text.decoder = get_codepage(self.text.code_table, self.text.character_set);
    }
}
