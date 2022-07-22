use crate::{command::*, constants::*};

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {}

pub fn new() -> Command {
    Command::new(
        "Set Code Table",
        vec![ESC, 't' as u8],
        CommandType::Context,
        DataType::Single,
        Box::new(Handler {}),
    )
}