//See: https://reference.epson-biz.com/modules/ref_escpos/index.php?content_id=130
use crate::parser::*;
use crate::parser::common_handlers::graphics_data;

//TODO implement custom data methods

pub fn new() -> Command {
  Command::new(
    "QR Code",
    vec![GS, '(' as u8, 'k' as u8], 
    CommandType::Graphics,
    DataType::Custom,
    graphics_data::new(false)
  )
}