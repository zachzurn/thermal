use crate::parser::*;

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {}

//Position of Human Readable characters
pub fn new() -> Command {
  Command::new(
    "Set Hri Print POS",
    vec![GS, 'H' as u8], 
    CommandType::GraphicsContext,
    DataType::Single,
    Box::new(Handler{})
  )
}