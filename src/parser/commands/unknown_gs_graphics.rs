use crate::parser::*;

pub fn new() -> Command {
  Command::new(
    "Unknown GS Graphics Command",
    vec![GS, '(' as u8], 
    CommandType::Unknown,
    DataType::Custom,
    subcommands::new(false, false, subcommands::no_commands())
  )
}