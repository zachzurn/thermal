//!
//! This command is used to set the y position in page mode.
//!
//! Page mode has the concept of print direction, thus we have
//! to set the x or y based on what the print direction is set to.
//!
//! The position cannot exceed the width or height that is set
//! in page mode.
//!
//! This command is only applicable when page mode is enabled.
//!
use crate::command::{Command, CommandHandler, CommandType, DataType};
use crate::constants::GS;
use crate::context::Context;

#[derive(Clone)]
struct Handler;

fn get_pos(data: &Vec<u8>) -> u32 {
    let nl = data.get(0).unwrap_or(&0u8);
    let nh = data.get(1).unwrap_or(&0u8);

    (*nl as u16 + *nh as u16 * 256) as u32
}

impl CommandHandler for Handler {
    fn apply_context(&self, command: &Command, context: &mut Context) {
        if context.page_mode.enabled {
            context.set_x(get_pos(&command.data));
        }
    }

    fn debug(&self, command: &Command, _context: &Context) -> String {
        format!("{} --> {}", &command.name, get_pos(&command.data))
    }
}

pub fn new() -> Command {
    Command::new(
        "Set Absolute Vertical Position", //should be JQuery Command :)
        vec![GS, '$' as u8],
        CommandType::Context,
        DataType::Double,
        Box::new(Handler {}),
    )
}
