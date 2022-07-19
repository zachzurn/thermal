use crate::parser::*;
use crate::parser::graphics::{Image, ImageRefStorage};

#[derive(Clone)]
pub struct Handler;

impl CommandHandler for Handler {
  fn apply_context(&self, command: &Command, context: &mut Context) {
    if let Some((img_ref, img)) = Image::from_table_data_with_ref(&command.data, ImageRefStorage::Disc){
      context.graphics.stored_graphics.insert(img_ref, img);
    }
  }
}

pub fn new() -> Command {
  Command::new(
    "Define NV Graphics in Column Format",
    vec![68], 
    CommandType::Subcommand,
    DataType::Subcommand,
    Box::new(Handler)
  )
}