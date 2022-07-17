use crate::parser::*;

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {}

pub fn new() -> Command {
  Command::new(
    "Set International Character Set",
    vec![ESC, 'R' as u8], 
    CommandType::Context,
    DataType::Single,
    Box::new(Handler{})
  )
}