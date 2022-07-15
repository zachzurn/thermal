use std::sync::Arc;

use crate::parser::*;

struct Handler;

impl CommandHandler for Handler {}

pub fn command() -> Command {
  Command::new(
    "Print and Feed",
    vec![ESC, 'J' as u8], 
    CommandType::Control,
    DataType::Single,
    Arc::new(Handler{})
  )
}