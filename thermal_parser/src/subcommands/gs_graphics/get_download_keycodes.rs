use crate::{command::*};

#[derive(Clone)]
pub struct Handler;

impl CommandHandler for Handler {
    //TODO implement transmit
}

//Transmits the defined NV graphics key code list.
pub fn new() -> Command {
    Command::new(
        "Get Download RAM Key Codes",
        vec![80],
        CommandType::Control,
        DataType::Subcommand,
        Box::new(Handler),
    )
}
