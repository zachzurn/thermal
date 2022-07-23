use crate::{command::*, constants::*};
use crate::context::Context;
use crate::graphics::GraphicsCommand;

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {
    fn apply_context(&self, _command: &Command, context: &mut Context) {
        context.reset_text_line_spacing();
    }
}

pub fn new() -> Command {
    Command::new(
        "Default Line Spacing",
        vec![ESC, '2' as u8],
        CommandType::Context,
        DataType::Empty,
        Box::new(Handler {}),
    )
}
