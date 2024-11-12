use crate::{command::*, constants::*, context::*};

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {
    fn apply_context(&self, command: &Command, context: &mut Context) {
        let n = *command.data.get(0).unwrap_or(&0u8);
        context.text.color = match n {
            1 | 49 => context.graphics.render_colors.color_2,
            _ => context.graphics.render_colors.color_1,
        }
    }
}

pub fn new() -> Command {
    Command::new(
        "Set Print Color to Alternate",
        vec![ESC, 'r' as u8],
        CommandType::TextStyle,
        DataType::Single,
        Box::new(Handler {}),
    )
}
