use std::sync::Arc;

use crate::parser::*;

struct Handler;

impl CommandHandler for Handler {}

pub fn new() -> Command {
  Command::new(
    "Set Hri Print POS",
    vec![GS, 'H' as u8], 
    CommandType::GraphicsContext,
    DataType::Single,
    Arc::new(Mutex::new(Handler{}))
  )
}