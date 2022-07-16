use std::sync::Arc;

use crate::parser::*;

struct Handler;

impl CommandHandler for Handler {}

pub fn new() -> Command {
  Command::new(
    "Unknown ESC C command",
    vec![ESC, 'c' as u8], 
    CommandType::Unknown,
    DataType::Single,
    Arc::new(Mutex::new(Handler{}))
  )
}