use crate::parser::*;

#[derive(Clone)]
pub struct Handler;

impl CommandHandler for Handler {
  fn apply_context(&self, command: &Command, context: &mut Context) {
    let m = *command.data.get(0).unwrap_or(&0u8);
    let d1 = *command.data.get(1).unwrap_or(&0u8);
    let d2 = *command.data.get(2).unwrap_or(&0u8);

    let mut columns = 0;
    let mut rows = 0;
    let mut symbol_type = 0;

    match m {
      0 | 48 => {
        if d1 + d2 > 0 {
          columns = d1;
          rows = d2;
        }
      }
      1 | 49 => {
        symbol_type = 2;
        if d2 == 0 && (d1 == 8 || d1 == 12 || d1 == 16) {
          columns = d1;
        } else {
          columns = d1;
          rows = d2;
        }
      }
      _ => { return }
    }

    context.code2d.datamatrix_rows = rows;
    context.code2d.datamatrix_columns = columns;

    //0 = square ECC200
    //1 = rectangle ECC200
    context.code2d.datamatrix_type = symbol_type;

  }
}

pub fn new() -> Command {
  Command::new(
    "Datamatrix Set Options",
    vec![54, 66],
    CommandType::Subcommand,
    DataType::Subcommand,
    Box::new(Handler)
  )
}