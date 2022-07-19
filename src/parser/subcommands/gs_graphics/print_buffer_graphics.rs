use crate::parser::*;

#[derive(Clone)]
pub struct Handler;

impl CommandHandler for Handler {
  fn get_graphics(&self, _command: &Command, context: &Context) -> Option<GraphicsCommand> {
      match &context.graphics.buffer_graphics {
        Some(img) => {
          return Some(GraphicsCommand::Image(img.clone()));
        }
        None => return None
      }
  }
}

pub fn new() -> Command {
  Command::new(
    "Print Buffer Graphics",
    vec![2, 50], 
    CommandType::Subcommand,
    DataType::Subcommand,
    Box::new(Handler)
  )
}