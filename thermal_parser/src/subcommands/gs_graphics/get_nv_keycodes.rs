use crate::command::*;

#[derive(Clone)]
pub struct Handler;

impl CommandHandler for Handler {
    //TODO implement transmit
}

//Transmits the defined NV graphics key code list.
pub fn new() -> Command {
    Command::new(
        "Get NV Key Codes",
        vec![64],
        CommandType::Control,
        DataType::Subcommand,
        Box::new(Handler),
    )
}
