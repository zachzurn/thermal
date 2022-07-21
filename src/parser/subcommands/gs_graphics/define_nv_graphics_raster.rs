use crate::parser::*;
use crate::parser::graphics::{Image, ImageRefStorage};

#[derive(Clone)]
pub struct Handler;

impl CommandHandler for Handler {
    fn apply_context(&self, command: &Command, context: &mut Context) {
        if let Some((img_ref, img)) = Image::from_raster_data_with_ref(&command.data, ImageRefStorage::Disc) {
            context.graphics.stored_graphics.insert(img_ref, img);
        }
    }
}

//Deletes the NV graphics data defined by the key codes (kc1 and kc2).
pub fn new() -> Command {
    Command::new(
        "Define NV Graphics in Raster Format",
        vec![67],
        CommandType::Subcommand,
        DataType::Subcommand,
        Box::new(Handler),
    )
}