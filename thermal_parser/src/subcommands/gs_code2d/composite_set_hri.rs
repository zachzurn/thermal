use crate::{command::*, context::*};

#[derive(Clone)]
pub struct Handler;

impl CommandHandler for Handler {
    fn apply_context(&self, command: &Command, context: &mut Context) {
        let n = *command.data.get(0).unwrap_or(&0u8);
        context.code2d.composite_font = Font::from_raw(n);
    }
}

pub fn new() -> Command {
    Command::new(
        "Composite Sets Human Readable Options",
        vec![52, 72],
        CommandType::Context,
        DataType::Subcommand,
        Box::new(Handler),
    )
}
