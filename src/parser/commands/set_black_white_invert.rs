use crate::parser::*;

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {}

pub fn new() -> Command {
    Command::new(
        "Set Black White Invert",
        vec![GS, 'B' as u8],
        CommandType::Context,
        DataType::Single,
        Box::new(Handler {}),
    )
}