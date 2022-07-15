use std::sync::Arc;

use crate::parser::*;

struct Handler;

impl CommandHandler for Handler {}

pub fn command() -> Command {
  Command::new(
    "Unknown GS Command",
    vec![GS], 
    CommandType::Unknown,
    DataType::Unknown,
    Arc::new(Handler{})
  )
}