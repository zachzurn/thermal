use std::sync::Arc;

use crate::parser::*;

struct Handler;

impl CommandHandler for Handler {}

pub fn command() -> Command {
  Command::new(
    "Set Peripheral Device",
    vec![ESC, '=' as u8], 
    CommandType::Control,
    DataType::Single,
    Arc::new(Handler{})
  )
}