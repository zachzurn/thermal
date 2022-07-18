use crate::parser::*;

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {
  fn get_text(&self, _command: &Command, _context: &Context) -> Option<String>{ 
    Some("\n".to_string())
  }
}

pub fn new() -> Command {
  Command::new(
    "Line Feed",
    vec![LF], 
    CommandType::Text,
    DataType::Empty,
    Box::new(Handler{})
  )
}