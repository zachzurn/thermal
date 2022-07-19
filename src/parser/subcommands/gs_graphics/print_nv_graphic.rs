use crate::parser::{*, graphics::{ImageRefStorage, ImageRef}};

#[derive(Clone)]
pub struct Handler;

impl CommandHandler for Handler {
  fn get_graphics(&self, command: &Command, context: &Context) -> Option<GraphicsCommand> {
    if let Some(img_ref) = ImageRef::from_data(&command.data, ImageRefStorage::Disc){
      if let Some(img) = context.graphics.stored_graphics.get(&img_ref){
        return Some(GraphicsCommand::Image(img.clone()));
      }
    }
    None
  }
}

//Deletes the NV graphics data defined by the key codes (kc1 and kc2).
pub fn new() -> Command {
  Command::new(
    "Print NV Graphic",
    vec![69], 
    CommandType::Subcommand,
    DataType::Subcommand,
    Box::new(Handler)
  )
}