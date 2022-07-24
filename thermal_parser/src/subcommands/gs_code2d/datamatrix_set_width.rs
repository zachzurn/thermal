use crate::{command::*, context::*};

#[derive(Clone)]
pub struct Handler;

impl CommandHandler for Handler {
    fn apply_context(&self, command: &Command, context: &mut Context) {
        context.code2d.datamatrix_width = *command.data.get(0).unwrap_or(&1u8);
    }
}

pub fn new() -> Command {
    Command::new(
        "Datamatrix Sets the dot Width",
        vec![52, 67],
        CommandType::Context,
        DataType::Subcommand,
        Box::new(Handler),
    )
}
