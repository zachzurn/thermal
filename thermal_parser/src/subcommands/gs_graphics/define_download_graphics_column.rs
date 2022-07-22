use crate::{command::*, context::*, graphics::*};

#[derive(Clone)]
pub struct Handler;

impl CommandHandler for Handler {
    fn apply_context(&self, command: &Command, context: &mut Context) {
        if let Some((img_ref, img)) = Image::from_table_data_with_ref(&command.data, ImageRefStorage::Ram) {
            context.graphics.stored_graphics.insert(img_ref, img);
        }
    }
}

//Deletes the NV graphics data defined by the key codes (kc1 and kc2).
pub fn new() -> Command {
    Command::new(
        "Define Download Graphics in Column Format",
        vec![84],
        CommandType::Subcommand,
        DataType::Subcommand,
        Box::new(Handler),
    )
}