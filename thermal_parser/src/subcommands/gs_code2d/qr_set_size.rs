use crate::{command::*, context::*};

#[derive(Clone)]
pub struct Handler;

impl CommandHandler for Handler {
    fn apply_context(&self, command: &Command, context: &mut Context) {
        context.code2d.qr_size = *command.data.get(0).unwrap_or(&1u8);
    }
}

pub fn new() -> Command {
    Command::new(
        "QR Sets the dot count",
        vec![49, 67],
        CommandType::Subcommand,
        DataType::Subcommand,
        Box::new(Handler),
    )
}