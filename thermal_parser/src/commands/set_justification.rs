use crate::{command::*, constants::*, context::*};

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {
    fn apply_context(&self, command: &Command, context: &mut Context) {
        let n = *command.data.get(0).unwrap_or(&0u8);
        context.text.justify = match n {
            0 | 48 => TextJustify::Left,
            1 | 49 => TextJustify::Center,
            2 | 50 => TextJustify::Right,
            _ => TextJustify::Left
        }
    }
}

pub fn new() -> Command {
    Command::new(
        "Set Text Justification",
        vec![ESC, 'a' as u8],
        CommandType::Context,
        DataType::Single,
        Box::new(Handler {}),
    )
}