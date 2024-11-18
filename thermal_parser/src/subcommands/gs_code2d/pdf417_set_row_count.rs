use crate::{command::*, context::*};

#[derive(Clone)]
pub struct Handler;

impl CommandHandler for Handler {
    fn apply_context(&self, command: &Command, context: &mut Context) {
        context.code2d.pdf417_rows = *command.data.get(0).unwrap_or(&0u8);
    }
}

pub fn new() -> Command {
    Command::new(
        "PDF417 Sets the number of rows",
        vec![48, 66],
        CommandType::Context,
        DataType::Subcommand,
        Box::new(Handler),
    )
}
