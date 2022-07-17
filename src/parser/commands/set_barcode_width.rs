use crate::parser::*;

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {}

pub fn new() -> Command {
  Command::new(
    "Set Barcode Width",
    vec![GS, 'w' as u8], 
    CommandType::GraphicsContext,
    DataType::Single,
    Box::new(Handler{})
  )
}