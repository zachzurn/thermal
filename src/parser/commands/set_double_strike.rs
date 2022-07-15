use std::sync::Arc;

use crate::parser::*;

struct Handler;

impl CommandHandler for Handler {}

pub fn command() -> Command {
  Command::new(
    "Enable Double Strike Through",
    vec![ESC, 'G' as u8], 
    CommandType::TextContext,
    DataType::Single,
    Arc::new(Handler{})
  )
}