use crate::parser::*;

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {}

pub fn new() -> Command {
  Command::new(
    "Enable Double Strike Through",
    vec![ESC, 'G' as u8], 
    CommandType::TextContext,
    DataType::Single,
    Box::new(Handler{})
  )
}