use crate::{command::*};

pub struct CommandSet {
    //list of supported commands
    pub commands: Box<Vec<Command>>,
    //default command (normally a text command)
    pub default: Command,
    //default command for catching unknown commands
    pub unknown: Command,
}

pub mod esc_pos;