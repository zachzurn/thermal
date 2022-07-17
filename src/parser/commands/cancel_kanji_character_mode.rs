use crate::parser::*;

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {}

pub fn new() -> Command {
  Command::new(
    "Cancel Kanji Character Mode",
    vec![FS, '.' as u8], 
    CommandType::Context,
    DataType::Empty,
    Box::new(Handler{})
  )
}