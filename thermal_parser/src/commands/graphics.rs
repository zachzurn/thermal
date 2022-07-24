use crate::{command::*, constants::*, subcommands};

pub fn new() -> Command {
    Command::new(
        "Graphics",
        vec![GS, '(' as u8, 'L' as u8],
        CommandType::Subcommand,
        DataType::Custom,
        subcommands::new(false, false, subcommands::gs_graphics::all()),
    )
}
