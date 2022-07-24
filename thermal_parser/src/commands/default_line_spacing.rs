use crate::{command::*, constants::*, context::*};

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {
    fn apply_context(&self, _command: &Command, context: &mut Context) {
        if let Some(default_context) = &context.default {
            context.text.line_spacing = default_context.text.line_spacing;
        }
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
