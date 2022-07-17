use crate::parser::*;

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {}

pub fn new() -> Command {
  Command::new(
    "Unknown DLE Command",
    vec![DLE], 
    CommandType::Unknown,
    DataType::Unknown,
    Box::new(Handler{})
  )
}


//Arc::new(Handler{}