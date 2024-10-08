use crate::{command::*, constants::*, context::*};

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {
    fn apply_context(&self, command: &Command, context: &mut Context) {
        let n = *command
            .data
            .get(0)
            .unwrap_or(&context.default.as_ref().unwrap().text.line_spacing);

        context.text.tab_len = n;
    }
}

pub fn new() -> Command {
    Command::new(
        "Set Tab Len",
        vec![ESC, 'D' as u8],
        CommandType::Context,
        DataType::Single,
        Box::new(Handler {}),
    )
}
