use crate::parser::*;

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {
  fn get_text(&self, _command: &Command) -> Option<String>{ 
    Some("\r".to_string())
  }
}

pub fn new() -> Command {
  Command::new(
    "Line Feed",
    vec![CR], 
    CommandType::Text,
    DataType::Empty,
    Box::new(Handler{})
  )
}