use crate::parser::*;

pub fn new() -> Command {
  Command::new(
    "Request Response Transmission",
    vec![GS, '(' as u8, 'H' as u8], 
    CommandType::Control,
    DataType::Custom,
    subcommands::new(false, false, subcommands::no_commands())
  )
}