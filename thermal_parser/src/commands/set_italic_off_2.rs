use crate::{command::*, constants::*, context::*};

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {
    fn apply_context(&self, _command: &Command, context: &mut Context) {
        context.text.italic = false;
    }
}

pub fn new() -> Command {
    Command::new(
        "Set Italic Off",
        vec![ESC, 0x34, 0x00],
        CommandType::Context,
        DataType::Empty,
        Box::new(Handler {}),
    )
}