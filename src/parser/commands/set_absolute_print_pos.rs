use std::sync::Arc;

use crate::parser::*;

struct Handler;

impl CommandHandler for Handler {}

pub fn command() -> Command {
  Command::new(
    "Set Absolute Print POS", //should be JQuery Command :)
    vec![ESC, '$' as u8], 
    CommandType::Control,
    DataType::Double,
    Arc::new(Handler{})
  )
}