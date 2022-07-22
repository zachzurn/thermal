use crate::{command::*, constants::*};

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {}

pub fn new() -> Command {
    Command::new(
        "Pulse",
        vec![ESC, 'p' as u8],
        CommandType::Control,
        DataType::Triple,
        Box::new(Handler {}),
    )
}