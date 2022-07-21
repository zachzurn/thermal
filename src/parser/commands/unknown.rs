use crate::parser::*;

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {}

pub fn new() -> Command {
    Command::new(
      "Unknown Command",
      vec![DLE, ESC, FS, GS],
      CommandType::Unknown,
      DataType::Unknown,
      Box::new(Handler {}),
    )
}


//Arc::new(Handler{}