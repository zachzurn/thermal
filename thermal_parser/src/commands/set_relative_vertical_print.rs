use crate::{command::*, constants::*};

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {}

pub fn new() -> Command {
    Command::new(
        "Set Relative Vertical Print",
        vec![GS, '\\' as u8],
        CommandType::Context,
        DataType::Double,
        Box::new(Handler {}),
    )
}