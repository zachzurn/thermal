use std::sync::Arc;

use crate::parser::*;

struct Handler;

impl CommandHandler for Handler {}

pub fn command() -> Command {
  Command::new(
    "Set Line Spacing",
    vec![ESC, '3' as u8], 
    CommandType::TextContext,
    DataType::Single,
    Arc::new(Handler{})
  )
}