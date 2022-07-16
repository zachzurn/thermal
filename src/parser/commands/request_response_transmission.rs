use crate::parser::*;
use crate::parser::common_handlers::data_handler;

pub fn new() -> Command {
  Command::new(
    "Request Response Transmission",
    vec![GS, '(' as u8, 'H' as u8], 
    CommandType::Control,
    DataType::Custom,
    data_handler::new(false)
  )
}