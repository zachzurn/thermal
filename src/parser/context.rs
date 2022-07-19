use super::graphics::{Image, ImageRef};
use std::collections::HashMap;

#[derive(Clone)]
pub enum TextJustify{ Left, Center, Right }

#[derive(Clone)]
pub enum TextUnderline{ Off, On, Double }

#[derive(Clone)]
pub enum Font{ A, B, C }


#[derive(Clone)]
pub struct Context {
  pub text: TextContext,
  pub barcode: BarcodeContext,
  pub code2d: Code2DContext,
  pub graphics: GraphicsContext
}

#[derive(Clone)]
pub struct TextContext {
    pub justify: TextJustify,
    pub font: Font,
    pub bold: bool,
    pub underline: TextUnderline,
    pub invert: bool,
    pub width_mult: u16,
    pub height_mult: u16,
    pub upside_down: bool, 
}

#[derive(Clone)]
pub struct GraphicsContext {
  pub dots_per_inch: u16,
  pub graphics_count: u16,
  pub stored_graphics: HashMap<ImageRef, Image>,
  pub buffer_graphics: Option<Image>
}

#[derive(Clone)]
pub struct BarcodeContext {
  pub human_readable: u8,
  pub width: u8,
  pub height: u8,
}

#[derive(Clone)]
pub struct Code2DContext {
   pub qr_model: u8,
   pub qr_size: u8,
   pub qr_err_correction: u8,
 
   pub pdf417_columns: u8,
   pub pdf417_rows: u8,
   pub pdf417_width: u8,
   pub pdf417_row_height: u8,
   pub pdf417_err_correction: u8,
   pub pdf417_is_truncated: u8,
 
   pub maxicode_mode: u8,
 
   pub gs1_databar_width: u8,
   pub gs1_databar_max_width: u8,
 
   pub composite_width: u8,
   pub composite_max_width: u8,
 
   pub aztec_mode: u8,
   pub aztec_layers: u8,
   pub aztec_size: u8,
   pub aztec_error_correction: u8,
 
   pub datamatrix_type: u8,
   pub datamatrix_columns: u8,
   pub datamatrix_rows: u8,
}

static TEXT_DEFAULT: TextContext = TextContext{
  justify: TextJustify::Left,
  font: Font::A,
  bold: false,
  underline: TextUnderline::Off,
  invert: false,
  width_mult: 100,
  height_mult: 100,
  upside_down: false
};

static BARCODE_DEFAULT: BarcodeContext = BarcodeContext{
    human_readable: 0,
    width: 2,
    height: 40,
};

static CODE2D_DEFAULT: Code2DContext = Code2DContext{
    qr_model: 0,
    qr_size: 0,
    qr_err_correction: 0,
    pdf417_columns: 0,
    pdf417_rows: 0,
    pdf417_width: 0,
    pdf417_row_height: 0,
    pdf417_err_correction: 0,
    pdf417_is_truncated: 0,
    maxicode_mode: 0,
    gs1_databar_width: 0,
    gs1_databar_max_width: 0,
    composite_width: 0,
    composite_max_width: 0,
    aztec_mode: 0,
    aztec_layers: 0,
    aztec_size: 0,
    aztec_error_correction: 0,
    datamatrix_type: 0,
    datamatrix_columns: 0,
    datamatrix_rows: 0,
};

static DEFAULT_GRAPHICS_DPI: u16 = 180;

impl Context {
  pub fn new() -> Context {
    Context { 
      text: TEXT_DEFAULT.clone(), 
      barcode: BARCODE_DEFAULT.clone(), 
      code2d: CODE2D_DEFAULT.clone(),
      graphics: GraphicsContext { 
          dots_per_inch: DEFAULT_GRAPHICS_DPI,
          graphics_count: 0,
          stored_graphics: HashMap::<ImageRef, Image>::new(),
          buffer_graphics: None,
      }
    }
  }

  pub fn reset_text_context(&mut self){
    self.text = TEXT_DEFAULT.clone();
  }

  pub fn reset(&mut self){
    self.text = TEXT_DEFAULT.clone();
    self.barcode = BARCODE_DEFAULT.clone();
    self.code2d = CODE2D_DEFAULT.clone();
    self.graphics.dots_per_inch = DEFAULT_GRAPHICS_DPI;
    self.graphics.graphics_count = 0;
  }
}