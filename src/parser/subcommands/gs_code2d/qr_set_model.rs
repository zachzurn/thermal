use crate::parser::*;

#[derive(Clone)]
pub struct Handler;

impl CommandHandler for Handler {
  fn apply_context(&self, command: &Command, context: &mut Context) {
    let n1 = *command.data.get(0).unwrap_or(&49u8);
    let _n2 = *command.data.get(1).unwrap_or(&0u8);
    let mut model = n1 - 48;
    if model == 3 { model = 0 }

    //0 = micro
    //1 = model 1
    //2 = model 2
    context.code2d.qr_model = model;
  }
}

pub fn new() -> Command {
  Command::new(
    "QR Sets the Model",
    vec![49, 65],
    CommandType::Subcommand,
    DataType::Subcommand,
    Box::new(Handler)
  )
}