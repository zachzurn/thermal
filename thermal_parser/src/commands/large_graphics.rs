use crate::{command::*, constants::*, subcommands};

pub fn new() -> Command {
    Command::new(
        "Large Graphics",
        vec![GS, '8' as u8, 'L' as u8],
        CommandType::Subcommand,
        DataType::Custom,
        subcommands::new(false, false, subcommands::gs_graphics::all()),
    )
}
