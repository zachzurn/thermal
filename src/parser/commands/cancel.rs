use std::sync::Arc;

use crate::parser::*;

struct Handler;

impl CommandHandler for Handler {}

pub fn new() -> Command {
  Command::new(
    "Initialize",
    vec![CAN, '@' as u8], 
    CommandType::Control,
    DataType::Empty,
    Arc::new(Mutex::new(Handler{}))
  )
}