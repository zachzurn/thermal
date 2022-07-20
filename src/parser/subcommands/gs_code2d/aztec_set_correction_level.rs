use crate::parser::*;

#[derive(Clone)]
pub struct Handler;

impl CommandHandler for Handler {
  fn apply_context(&self, command: &Command, context: &mut Context) {
    let n = *command.data.get(0).unwrap_or(&5u8);
    context.code2d.aztec_error_correction = n;
  }
}

pub fn new() -> Command {
  Command::new(
    "Aztec Set Error Correction Level",
    vec![53, 69],
    CommandType::Subcommand,
    DataType::Subcommand,
    Box::new(Handler)
  )
}