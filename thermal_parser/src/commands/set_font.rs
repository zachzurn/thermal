use crate::{command::*, constants::*, context::*};

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {
    fn apply_context(&self, command: &Command, context: &mut Context) {
        let n = *command.data.get(0).unwrap_or(&0u8);
        context.set_font(Font::from_raw(n));
    }
}

pub fn new() -> Command {
    Command::new(
        "Set Font",
        vec![ESC, 'M' as u8],
        CommandType::Context,
        DataType::Single,
        Box::new(Handler {}),
    )
}
