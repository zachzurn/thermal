use crate::{command::*};

pub struct CommandSet {
    //list of supported commands
    pub commands: Box<Vec<Command>>,
    //default command (normally a text command)
    pub default: Command,
    //default command for catching unknown commands
    pub unknown: Command,

    //Used for indicating the beginning end ending of parsing
    pub begin_parsing: Command,
    pub end_parsing: Command
}

pub mod esc_pos;
