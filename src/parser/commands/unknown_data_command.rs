use crate::parser::*;

pub fn new() -> Command {
  Command::new(
    "Uknown Data",
    vec![GS, '(' as u8, 'J' as u8], 
    CommandType::Unknown,
    DataType::Custom,
    subcommands::new(false, false, subcommands::no_commands())
  )
}