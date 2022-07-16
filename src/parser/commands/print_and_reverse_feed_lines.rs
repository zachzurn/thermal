use std::sync::Arc;

use crate::parser::*;

struct Handler;

impl CommandHandler for Handler {}

pub fn new() -> Command {
  Command::new(
    "Print and Reverse Feed Lines",
    vec![ESC, 'e' as u8], 
    CommandType::Control,
    DataType::Single,
    Arc::new(Mutex::new(Handler{}))
  )
}