use crate::command::*;

#[derive(Clone)]
pub struct Handler;

impl CommandHandler for Handler {
    //TODO implement transmit
}

//Transmits the entire capacity of the NV graphics area (number of bytes in the NV graphics area).
pub fn new() -> Command {
    Command::new(
        "Get NV Capacity",
        vec![0, 48],
        CommandType::Control,
        DataType::Subcommand,
        Box::new(Handler),
    )
}
