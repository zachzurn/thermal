use crate::{command::*, context::*};

#[derive(Clone)]
pub struct Handler;

impl CommandHandler for Handler {
    fn apply_context(&self, command: &Command, context: &mut Context) {
        context.code2d.pdf417_width = *command.data.get(0).unwrap_or(&0u8);
    }
}

pub fn new() -> Command {
    Command::new(
        "PDF417 Sets the Column Width",
        vec![48, 67],
        CommandType::Subcommand,
        DataType::Subcommand,
        Box::new(Handler),
    )
}