pub static TEXT_LEFT: u8 = 0;
pub static TEXT_CENTER: u8 = 1;
pub static TEXT_RIGHT: u8 = 2;


pub struct TextContext {
  justify: u8,
  font: u8,
  bold: bool,
  underline: bool,
  invert: bool,
  upside_down: bool,
  width_multiplier: u16,
  height_multiplier: u16
}

impl TextContext {
  pub fn new() -> TextContext {
    TextContext { 
      justify: TEXT_LEFT,
      font: 0,
      bold: false,
      underline: false,
      invert: false,
      upside_down: false,
      width_multiplier: 100,
      height_multiplier: 100
    }
  }

  pub fn reset(&mut self) {
    self.justify = TEXT_LEFT;
    self.font = 0;
    self.bold = false;
    self.underline = false;
    self.invert = false;
    self.upside_down = false;
    self.width_multiplier = 100;
    self.height_multiplier = 100;
  }
}