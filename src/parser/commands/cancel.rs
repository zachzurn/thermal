use std::sync::Arc;

use crate::parser::*;

struct Handler;

impl CommandHandler for Handler {}

pub fn command() -> Command {
  Command::new(
    "Initialize",
    vec![CAN, '@' as u8], 
    CommandType::Control,
    DataType::Empty,
    Arc::new(Handler{})
  )
}