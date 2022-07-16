use std::sync::Arc;

use crate::parser::*;

struct Handler;

impl CommandHandler for Handler {
  fn get_text(&self, _command: &Command) -> Option<String>{ 
    Some("\n".to_string())
  }
}

pub fn new() -> Command {
  Command::new(
    "Line Feed",
    vec![LF], 
    CommandType::Text,
    DataType::Empty,
    Arc::new(Mutex::new(Handler{}))
  )
}