use std::sync::Arc;

use crate::parser::*;

struct Handler;

impl CommandHandler for Handler {}

pub fn command() -> Command {
  Command::new(
    "Unknown ESC C command A",
    vec![ESC, 'c' as u8, 0u8], 
    CommandType::Control,
    DataType::Single,
    Arc::new(Handler{})
  )
}