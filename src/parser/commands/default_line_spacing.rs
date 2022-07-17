use crate::parser::*;

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {}

pub fn new() -> Command {
  Command::new(
    "Default Line Spacing",
    vec![ESC, '2' as u8], 
    CommandType::Context,
    DataType::Empty,
    Box::new(Handler{})
  )
}