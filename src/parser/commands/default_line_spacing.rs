use std::sync::Arc;

use crate::parser::*;

struct Handler;

impl CommandHandler for Handler {}

pub fn command() -> Command {
  Command::new(
    "Default Line Spacing",
    vec![ESC, '2' as u8], 
    CommandType::TextContext,
    DataType::Empty,
    Arc::new(Handler{})
  )
}