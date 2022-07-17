use crate::parser::*;
use crate::parser::common_handlers::graphics_data;

pub fn new() -> Command {
  Command::new(
    "Request Response Transmission",
    vec![GS, '(' as u8, 'H' as u8], 
    CommandType::Control,
    DataType::Custom,
    graphics_data::new(false)
  )
}