use crate::parser::*;
use crate::parser::common_handlers::graphics_data;

struct Handler;

impl CommandHandler for Handler {
  
}

pub fn new() -> Command {
  Command::new(
    "Graphics",
    vec![GS, '(' as u8, 'L' as u8], 
    CommandType::Graphics,
    DataType::Custom,
    graphics_data::new(false)
  )
}