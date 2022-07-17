use crate::parser::*;
use crate::parser::common_handlers::graphics_data;

pub fn new() -> Command {
  Command::new(
    "Large Graphics",
    vec![GS, '8' as u8, 'L' as u8], 
    CommandType::Graphics,
    DataType::Custom,
    graphics_data::new(true)
  )
}