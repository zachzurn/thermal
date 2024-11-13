use crate::{command::*, constants::*, context::*};

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {
    fn apply_context(&self, command: &Command, context: &mut Context) {
        let n = *command.data.get(0).unwrap_or(&0u8);

        if n == 1 {
            context.text.background_color = context.graphics.render_colors.color_1;
            context.text.color = context.graphics.render_colors.paper_color;
        } else {
            context.text.background_color = context.graphics.render_colors.paper_color;
            context.text.color = context.graphics.render_colors.color_1;
        }

        context.text.invert = n == 1;
    }
}

pub fn new() -> Command {
    Command::new(
        "Set Black White Invert",
        vec![GS, 'B' as u8],
        CommandType::TextStyle,
        DataType::Single,
        Box::new(Handler {}),
    )
}
