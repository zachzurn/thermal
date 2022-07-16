use std::sync::Arc;

use crate::parser::*;

struct Handler;

impl CommandHandler for Handler {
  fn push(&mut self, data: &mut Vec<u8>, byte: u8) -> bool{ 
    if data.len() == 0 { data.push(byte) };
    if data.len() == 1 { 
      match data.get(0).unwrap() {
        0u8 | 48u8 | 1u8 | 49u8 => return false,
        _default => data.push(byte)
      }
    };
    false
  }
}

pub fn new() -> Command {
  Command::new(
    "Feed and Cut",
    vec![GS, 'V' as u8], 
    CommandType::Control,
    DataType::Custom, //push is implemented in the CommandHandler for Custom types
    Arc::new(Mutex::new(Handler{}))
  )
}