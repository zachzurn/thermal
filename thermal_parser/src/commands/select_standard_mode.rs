//!
//! This command ends Page Mode.
//!
//! The contents of the Page Mode buffer are not printed
//!

use crate::context::Context;
use crate::{command::*, constants::*};

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {
    fn apply_context(&self, _command: &Command, context: &mut Context) {
        context.page_mode.enabled = false;
    }

    fn get_device_command(
        &self,
        _command: &Command,
        context: &Context,
    ) -> Option<Vec<DeviceCommand>> {
        if context.page_mode.enabled {
            return Some(vec![DeviceCommand::EndPageMode(false)]);
        }
        None
    }
}

pub fn new() -> Command {
    Command::new(
        "Select standard mode",
        vec![ESC, 'S' as u8],
        CommandType::Context,
        DataType::Empty,
        Box::new(Handler {}),
    )
}
