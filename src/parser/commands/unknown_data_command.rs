use crate::parser::*;
use crate::parser::common_handlers::graphics_data;

pub fn new() -> Command {
  Command::new(
    "Uknown Data",
    vec![GS, '(' as u8, 'J' as u8], 
    CommandType::Unknown,
    DataType::Custom,
    graphics_data::new(false)
  )
}