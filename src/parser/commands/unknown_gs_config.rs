use crate::parser::*;
use crate::parser::common_handlers::graphics_data;

pub fn new() -> Command {
  Command::new(
    "Unknown GS User Configuration Command",
    vec![GS, 'E' as u8], 
    CommandType::Unknown,
    DataType::Custom,
    graphics_data::new(false)
  )
}