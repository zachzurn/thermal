use crate::parser::*;

#[derive(Clone)]
pub struct Handler;

impl CommandHandler for Handler {
  fn apply_context(&self, command: &Command, context: &mut Context) {
      context.code2d.composite_width = *command.data.get(0).unwrap_or(&1u8);
  }
}

pub fn new() -> Command {
  Command::new(
    "Composite Sets the dot Width",
    vec![52, 67],
    CommandType::Subcommand,
    DataType::Subcommand,
    Box::new(Handler)
  )
}