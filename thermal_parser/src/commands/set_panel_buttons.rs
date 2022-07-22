use crate::{command::*, constants::*};

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {}

pub fn new() -> Command {
    Command::new(
        "Enable Panel Buttons",
        vec![ESC, 'c' as u8, 5u8],
        CommandType::Control,
        DataType::Single,
        Box::new(Handler {}),
    )
}