use std::sync::Arc;

use crate::parser::*;

struct Handler;

impl CommandHandler for Handler {}

pub fn new() -> Command {
  Command::new(
    "Set Barcode Height",
    vec![GS, 'h' as u8], 
    CommandType::GraphicsContext,
    DataType::Single,
    Arc::new(Mutex::new(Handler{}))
  )
}