use std::sync::Arc;

use crate::parser::*;

struct Handler;

impl CommandHandler for Handler {}

pub fn new() -> Command {
  Command::new(
    "Pulse",
    vec![ESC, 'p' as u8], 
    CommandType::Control,
    DataType::Triple,
    Arc::new(Mutex::new(Handler{}))
  )
}