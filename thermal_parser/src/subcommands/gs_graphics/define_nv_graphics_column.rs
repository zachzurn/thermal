use crate::{command::*, context::*, graphics::*};

#[derive(Clone)]
pub struct Handler;

impl CommandHandler for Handler {
    fn apply_context(&self, command: &Command, context: &mut Context) {
        if let Some((img_ref, mut img)) = Image::from_column_data_with_ref(
            &command.data,
            ImageRefStorage::Disc,
            &context.graphics.render_colors,
        ) {
            img.flow = ImageFlow::Block;
            context.graphics.stored_graphics.insert(img_ref, img);
        }
    }
}

pub fn new() -> Command {
    Command::new(
        "Define NV Graphics in Column Format",
        vec![68],
        CommandType::Context,
        DataType::Subcommand,
        Box::new(Handler),
    )
}
