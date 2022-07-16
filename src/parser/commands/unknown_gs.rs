use std::sync::Arc;

use crate::parser::*;

struct Handler;

impl CommandHandler for Handler {}

pub fn new() -> Command {
  Command::new(
    "Unknown GS Command",
    vec![GS], 
    CommandType::Unknown,
    DataType::Unknown,
    Arc::new(Mutex::new(Handler{}))
  )
}