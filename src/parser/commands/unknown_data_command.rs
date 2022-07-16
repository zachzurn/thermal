use crate::parser::*;
use crate::parser::common_handlers::data_handler;

pub fn new() -> Command {
  Command::new(
    "Uknown Data",
    vec![GS, '(' as u8, 'J' as u8], 
    CommandType::Unknown,
    DataType::Custom,
    data_handler::new(false)
  )
}