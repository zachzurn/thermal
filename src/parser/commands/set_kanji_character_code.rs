use crate::parser::*;

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {}

pub fn new() -> Command {
  Command::new(
    "Set Kanji Character Code",
    vec![FS, 'C' as u8], 
    CommandType::TextContext,
    DataType::Single,
    Box::new(Handler{})
  )
}