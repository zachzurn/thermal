use crate::parser::*;

pub fn new() -> Command {
  Command::new(
    "Large Graphics",
    vec![GS, '8' as u8, 'L' as u8], 
    CommandType::Graphics,
    DataType::Custom,
    subcommands::new(false, subcommands::gs_graphics::all())
  )
}