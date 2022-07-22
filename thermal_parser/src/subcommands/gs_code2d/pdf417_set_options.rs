use crate::{command::*, context::*};

#[derive(Clone)]
pub struct Handler;

impl CommandHandler for Handler {
    fn apply_context(&self, command: &Command, context: &mut Context) {
        let n = *command.data.get(0).unwrap_or(&0u8);
        let truncated = if n == 0 { false } else { true };
        context.code2d.pdf417_is_truncated = truncated;
    }
}

pub fn new() -> Command {
    Command::new(
        "PDF417 Set Options",
        vec![48, 70],
        CommandType::Subcommand,
        DataType::Subcommand,
        Box::new(Handler),
    )
}