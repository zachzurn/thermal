use crate::{command::*, constants::*};

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {}

pub fn new() -> Command {
    Command::new(
        "Set Line Spacing",
        vec![ESC, '3' as u8],
        CommandType::Context,
        DataType::Single,
        Box::new(Handler {}),
    )
}