use crate::parser::{*, graphics::{ImageRef, ImageRefStorage}};

#[derive(Clone)]
pub struct Handler;

impl CommandHandler for Handler {
    fn apply_context(&self, command: &Command, context: &mut Context) {
        if let Some(img_ref) = ImageRef::from_data(&command.data, ImageRefStorage::Disc) {
            context.graphics.stored_graphics.remove(&img_ref);
        }
    }
}

//Deletes the NV graphics data defined by the key codes (kc1 and kc2).
pub fn new() -> Command {
    Command::new(
        "Clear NV Graphic",
        vec![66],
        CommandType::Subcommand,
        DataType::Subcommand,
        Box::new(Handler),
    )
}