use crate::parser::*;

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {}

pub fn new() -> Command {
    Command::new(
        "Transmit Printer ID",
        vec![GS, 'I' as u8],
        CommandType::Control,
        DataType::Single,
        Box::new(Handler {}),
    )
}