use std::sync::Arc;

use crate::parser::*;

struct Handler;

impl CommandHandler for Handler {}

pub fn command() -> Command {
  Command::new(
    "Enable Panel Buttons",
    vec![ESC, 'c' as u8, 5u8], 
    CommandType::Control,
    DataType::Single,
    Arc::new(Handler{})
  )
}