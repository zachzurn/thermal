use crate::parser::{*, graphics::{ImageRefStorage, ImageRef}};

#[derive(Clone)]
pub struct Handler;

impl CommandHandler for Handler {
    fn get_graphics(&self, command: &Command, context: &Context) -> Option<GraphicsCommand> {
        if let Some(img_ref) = ImageRef::from_data(&command.data, ImageRefStorage::Ram) {
            if let Some(img) = context.graphics.stored_graphics.get(&img_ref) {
                return Some(GraphicsCommand::Image(img.clone()));
            }
        }
        None
    }
}

pub fn new() -> Command {
    Command::new(
        "Print Download (RAM) Graphics",
        vec![85],
        CommandType::Subcommand,
        DataType::Subcommand,
        Box::new(Handler),
    )
}