use crate::parser::*;

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {}

pub fn new() -> Command {
  Command::new(
    "Set Font",
    vec![ESC, 'M' as u8], 
    CommandType::TextContext,
    DataType::Single,
    Box::new(Handler{})
  )
}