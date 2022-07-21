use crate::parser::{*, graphics::{ImageRef, ImageRefStorage}};

#[derive(Clone)]
pub struct Handler;

impl CommandHandler for Handler {
    fn apply_context(&self, command: &Command, context: &mut Context) {
        if let Some(img_ref) = ImageRef::from_data(&command.data, ImageRefStorage::Ram) {
            context.graphics.stored_graphics.remove(&img_ref);
        }
    }
}

pub fn new() -> Command {
    Command::new(
        "Clear Download (RAM) Graphic",
        vec![82],
        CommandType::Subcommand,
        DataType::Subcommand,
        Box::new(Handler),
    )
}