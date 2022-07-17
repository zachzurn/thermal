use crate::parser::*;

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {
  
}

pub fn new() -> Command {
  Command::new(
    "Set Graphics X and Y Position",
    vec![GS, 'P' as u8], 
    CommandType::GraphicsContext,
    DataType::Double,
    Box::new(Handler{})
  )
}