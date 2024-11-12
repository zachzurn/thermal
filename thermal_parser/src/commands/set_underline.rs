use crate::{command::*, constants::*, context::*};

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {
    fn apply_context(&self, command: &Command, context: &mut Context) {
        let n = *command.data.get(0).unwrap_or(&0u8);
        match n {
            0 | 48 => context.text.underline = TextUnderline::Off,
            1 | 49 => context.text.underline = TextUnderline::On,
            2 | 50 => context.text.underline = TextUnderline::Double,
            _ => context.text.underline = TextUnderline::Off,
        }
    }
}

pub fn new() -> Command {
    Command::new(
        "Enable Underline",
        vec![ESC, '-' as u8],
        CommandType::TextStyle,
        DataType::Single,
        Box::new(Handler {}),
    )
}
