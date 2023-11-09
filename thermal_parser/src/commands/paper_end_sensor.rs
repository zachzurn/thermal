use crate::{command::*, constants::*};

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {}

pub fn new() -> Command {
    Command::new(
        "Select Paper End Sensors",
        vec![ESC, 'c' as u8, 3u8],
        CommandType::Control,
        DataType::Single,
        Box::new(Handler {}),
    )
}
