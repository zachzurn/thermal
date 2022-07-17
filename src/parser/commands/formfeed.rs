use crate::parser::*;

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {
  fn get_text(&self, _command: &Command) -> Option<String>{ 
    Some("\x0c".to_string())
  }
}

pub fn new() -> Command {
  Command::new(
    "Form Feed",
    vec![FF], 
    CommandType::Text,
    DataType::Empty,
    Box::new(Handler{})
  )
}