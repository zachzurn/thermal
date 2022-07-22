use crate::{command::*, constants::*};

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {}

pub fn new() -> Command {
    Command::new(
        "Set Barcode Width",
        vec![GS, 'w' as u8],
        CommandType::Context,
        DataType::Single,
        Box::new(Handler {}),
    )
}