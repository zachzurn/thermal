use crate::parser::*;

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {
  fn get_text(&self, _command: &Command) -> Option<String>{ 
    Some("\t".to_string())
  }
}

pub fn new() -> Command {
  Command::new(
    "Horizontal Tab",
    vec![HT], 
    CommandType::Text,
    DataType::Empty,
    Box::new(Handler{})
  )
}