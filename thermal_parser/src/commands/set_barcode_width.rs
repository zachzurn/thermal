use crate::{command::*, constants::*, context::*};

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {
    fn apply_context(&self, command: &Command, context: &mut Context) {
        let n = *command.data.get(0).unwrap_or(&0u8);
        context.barcode.width = n;
    }
}

pub fn new() -> Command {
    Command::new(
        "Set Barcode Width",
        vec![GS, 'w' as u8],
        CommandType::Context,
        DataType::Single,
        Box::new(Handler {}),
    )
}
