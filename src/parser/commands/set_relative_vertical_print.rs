use std::sync::Arc;

use crate::parser::*;

struct Handler;

impl CommandHandler for Handler {}

pub fn new() -> Command {
  Command::new(
    "Set Relative Vertical Print",
    vec![GS, '\\' as u8], 
    CommandType::TextContext,
    DataType::Double,
    Arc::new(Mutex::new(Handler{}))
  )
}