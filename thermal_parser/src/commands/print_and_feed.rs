use crate::{command::*, constants::*};

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {}

pub fn new() -> Command {
    Command::new(
        "Print and Feed",
        vec![ESC, 'J' as u8],
        CommandType::Control,
        DataType::Single,
        Box::new(Handler {}),
    )
}