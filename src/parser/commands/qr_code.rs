use crate::parser::*;
use crate::parser::common_handlers::data_handler;

//Notes here 
//$cn . $fn . $m . $data

//First byte of data is output code type
//Second byte is function to use
//Third byte is modifier Often 0

pub fn new() -> Command {
  Command::new(
    "QR Code",
    vec![GS, '(' as u8, 'k' as u8], 
    CommandType::Graphics,
    DataType::Custom,
    data_handler::new(false)
  )
}