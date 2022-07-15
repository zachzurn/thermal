use std::sync::Arc;

use crate::parser::*;

struct Handler;

impl CommandHandler for Handler {}

pub fn command() -> Command {
  Command::new(
    "Set Code Table",
    vec![ESC, 't' as u8], 
    CommandType::TextContext,
    DataType::Single,
    Arc::new(Handler{})
  )
}