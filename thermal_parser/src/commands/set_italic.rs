use crate::{command::*, constants::*, context::*};

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {
    fn apply_context(&self, command: &Command, context: &mut Context) {
        let n = *command.data.get(0).unwrap_or(&0u8);
        context.text.italic = (n & 0x00000001) == 1;
    }
}

pub fn new() -> Command {
    Command::new(
        "Enable Italic",
        vec![ESC, 0x34],
        CommandType::Context,
        DataType::Single,
        Box::new(Handler {}),
    )
}
