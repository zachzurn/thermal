use crate::{command::*};

#[derive(Clone)]
pub struct Handler;

impl CommandHandler for Handler {
    //TODO implement transmit
}

pub fn new() -> Command {
    Command::new(
        "Transmit Size of storage area",
        vec![50, 82],
        CommandType::Subcommand,
        DataType::Subcommand,
        Box::new(Handler),
    )
}