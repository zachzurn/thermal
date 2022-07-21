use crate::parser::*;

pub fn new() -> Command {
    Command::new(
        "Graphics",
        vec![GS, '(' as u8, 'L' as u8],
        CommandType::Graphics,
        DataType::Custom,
        subcommands::new(false, false, subcommands::gs_graphics::all()),
    )
}