use crate::parser::*;

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {}

pub fn new() -> Command {
  Command::new(
    "Set Relative Vertical Print",
    vec![GS, '\\' as u8], 
    CommandType::TextContext,
    DataType::Double,
    Box::new(Handler{})
  )
}