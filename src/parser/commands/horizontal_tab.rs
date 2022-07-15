use std::sync::Arc;

use crate::parser::*;

struct Handler;

impl CommandHandler for Handler {
  fn get_text(&self, _command: &Command) -> Option<String>{ 
    Some("\t".to_string())
  }
}

pub fn command() -> Command {
  Command::new(
    "Horizontal Tab",
    vec![HT], 
    CommandType::Text,
    DataType::Empty,
    Arc::new(Handler{})
  )
}