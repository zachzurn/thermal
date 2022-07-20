use crate::parser::*;

#[derive(Clone)]
pub struct Handler;

impl CommandHandler for Handler {
  fn apply_context(&self, command: &Command, context: &mut Context) {
    let n = *command.data.get(0).unwrap_or(&0u8);

    let font = match n {
      1 | 49 => 1, //1 = Font A
      2 | 50 => 2, //2 = Font B
      3 | 51 => 3, //3 = Font C
      4 | 52 => 4, //4 = Font D
      5 | 53 => 5, //5 = Font E
      97 => 6, //6 = Special Font A
      98 => 7, //7 = Special Font B
      _ => 0 //HRI characters are not added
    };

    context.code2d.composite_font = font;
  }
}

pub fn new() -> Command {
  Command::new(
    "Composite Sets Human Readable Options",
    vec![52, 72],
    CommandType::Subcommand,
    DataType::Subcommand,
    Box::new(Handler)
  )
}