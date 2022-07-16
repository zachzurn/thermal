use crate::parser::*;

struct Handler;

impl CommandHandler for Handler {
  
}

pub fn new() -> Command {
  Command::new(
    "Set Graphics X and Y Position",
    vec![GS, 'P' as u8], 
    CommandType::GraphicsContext,
    DataType::Double,
    Arc::new(Mutex::new(Handler{}))
  )
}