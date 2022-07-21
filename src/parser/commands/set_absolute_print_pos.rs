use crate::parser::*;

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {}

pub fn new() -> Command {
    Command::new(
        "Set Absolute Print POS", //should be JQuery Command :)
        vec![ESC, '$' as u8],
        CommandType::Control,
        DataType::Double,
        Box::new(Handler {}),
    )
}