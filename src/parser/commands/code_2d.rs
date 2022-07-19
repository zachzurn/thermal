//See: https://reference.epson-biz.com/modules/ref_escpos/index.php?content_id=130
use crate::parser::*;

//TODO implement custom data methods

pub fn new() -> Command {
  Command::new(
    "QR Code",
    vec![GS, '(' as u8, 'k' as u8], 
    CommandType::Graphics,
    DataType::Custom,
    subcommands::new(false, subcommands::gs_code2d::all())
  )
}