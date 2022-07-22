use crate::{command::*, constants::*};

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {}

pub fn new() -> Command {
    Command::new(
        "Enable Double Strike Through",
        vec![ESC, 'G' as u8],
        CommandType::Context,
        DataType::Single,
        Box::new(Handler {}),
    )
}