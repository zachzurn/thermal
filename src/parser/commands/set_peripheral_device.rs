use crate::parser::*;

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {}

pub fn new() -> Command {
    Command::new(
        "Set Peripheral Device",
        vec![ESC, '=' as u8],
        CommandType::Control,
        DataType::Single,
        Box::new(Handler {}),
    )
}