use crate::{command::*, constants::*};

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {}

pub fn new() -> Command {
    Command::new(
        "Initialize",
        vec![ESC, '@' as u8],
        CommandType::Initialize,
        DataType::Empty,
        Box::new(Handler {}),
    )
}