use std::sync::Arc;

use crate::parser::*;

struct Handler;

impl CommandHandler for Handler {}

pub fn command() -> Command {
  Command::new(
    "Image",
    vec![ESC, '{' as u8], 
    CommandType::Image,
    DataType::Bitmap,
    Arc::new(Handler{})
  )
}