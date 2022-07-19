use crate::parser::*;
use crate::parser::graphics::{Image};

#[derive(Clone)]
pub struct Handler;

impl CommandHandler for Handler {
  fn apply_context(&self, command: &Command, context: &mut Context) {
    if let Some(img) = Image::from_raster_data(&command.data){
      context.graphics.buffer_graphics = Some(img)
    }
  }
}

//Deletes the Download graphics data defined by the key codes (kc1 and kc2).
pub fn new() -> Command {
  Command::new(
    "Store Print Buffer Graphics Raster Format",
    vec![112], 
    CommandType::Subcommand,
    DataType::Subcommand,
    Box::new(Handler)
  )
}