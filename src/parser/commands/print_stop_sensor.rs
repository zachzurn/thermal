use std::sync::Arc;

use crate::parser::*;

struct Handler;

impl CommandHandler for Handler {}

pub fn new() -> Command {
  Command::new(
    "Print Stop Sensors",
    vec![ESC, 'c' as u8, 4u8], 
    CommandType::Control,
    DataType::Single,
    Arc::new(Mutex::new(Handler{}))
  )
}