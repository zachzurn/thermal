use crate::parser::*;

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {}

pub fn new() -> Command {
  Command::new(
    "Unknown FS Command",
    vec![FS], 
    CommandType::Unknown,
    DataType::Unknown,
    Box::new(Handler{})
  )
}