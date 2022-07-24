use crate::{command::*, constants::*, context::*};

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {
    fn apply_context(&self, command: &Command, context: &mut Context) {
        let n = *command.data.get(0).unwrap_or(&0u8);
        context.text.color = match n {
            1 | 49 => Color::Red,
            _ => Color::Black
        }
    }
}

pub fn new() -> Command {
    Command::new(
        "Set Alernate Color",
        vec![ESC, 'r' as u8],
        CommandType::Context,
        DataType::Single,
        Box::new(Handler {}),
    )
}
