use std::sync::Arc;

use crate::parser::*;

struct Handler;

impl CommandHandler for Handler {}

pub fn new() -> Command {
  Command::new(
    "Unknown DLE Command",
    vec![DLE], 
    CommandType::Unknown,
    DataType::Unknown,
    Arc::new(Mutex::new(Handler{}))
  )
}


//Arc::new(Handler{}