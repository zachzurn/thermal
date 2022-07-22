use crate::{command::*, constants::*};

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {}

pub fn new() -> Command {
    Command::new(
        "Set Alernate Color",
        vec![ESC, 'r' as u8],
        CommandType::Context,
        DataType::Single,
        Box::new(Handler {}),
    )
}