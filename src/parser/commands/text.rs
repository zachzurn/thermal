use std::sync::Arc;
use std::str::from_utf8;

use crate::parser::*;

struct Handler;

impl CommandHandler for Handler {
  fn get_text(&self, command: &Command) -> Option<String>{ 
    match from_utf8(&command.data as &[u8]) {
        Ok(str) => return Some(str.to_string()),
        Err(err) => { 
          print!("UTF8 ERROR {} {:02X?}", err, &command.data);
          return None 
        },
    };
  }
}

pub fn new() -> Command {
  Command::new(
    "Text",
    vec![], 
    CommandType::Text,
    DataType::Text,
    Arc::new(Mutex::new(Handler{}))
  )
}