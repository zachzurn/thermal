use crate::{command::*, context::*, graphics::*};

#[derive(Clone)]
pub struct Handler;

impl CommandHandler for Handler {
    fn apply_context(&self, command: &Command, context: &mut Context) {
        if let Some(mut img) =
            Image::from_column_data(&command.data, &context.graphics.render_colors)
        {
            img.flow = ImageFlow::Block;
            context.graphics.buffer_graphics.push(img)
        }
    }
}

pub fn new() -> Command {
    Command::new(
        "Store Print Buffer Graphics Table Format",
        vec![113],
        CommandType::Context,
        DataType::Subcommand,
        Box::new(Handler),
    )
}
