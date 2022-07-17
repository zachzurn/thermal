use crate::parser::*;

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {}

pub fn new() -> Command {
  Command::new(
    "Unknown ESC Command",
    vec![ESC], 
    CommandType::Unknown,
    DataType::Unknown,
    Box::new(Handler{})
  )
}