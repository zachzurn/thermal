use crate::parser::*;

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {}

pub fn new() -> Command {
  Command::new(
    "Initialize",
    vec![ESC, '@' as u8], 
    CommandType::Control,
    DataType::Empty,
    Box::new(Handler{})
  )
}