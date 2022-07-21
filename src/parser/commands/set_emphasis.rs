use crate::parser::*;

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {
    fn apply_context(&self, command: &Command, context: &mut Context) {
        let n = *command.data.get(0).unwrap_or(&0u8);
        context.text.bold = if n == 0x00 { false } else { true };
    }
}

pub fn new() -> Command {
    Command::new(
        "Enable Emphasis",
        vec![ESC, 'E' as u8],
        CommandType::Context,
        DataType::Single,
        Box::new(Handler {}),
    )
}