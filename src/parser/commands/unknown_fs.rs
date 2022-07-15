use std::sync::Arc;

use crate::parser::*;

struct Handler;

impl CommandHandler for Handler {}

pub fn command() -> Command {
  Command::new(
    "Unknown FS Command",
    vec![FS], 
    CommandType::Unknown,
    DataType::Unknown,
    Arc::new(Handler{})
  )
}