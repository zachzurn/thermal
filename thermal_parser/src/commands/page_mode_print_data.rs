//!
//! This command ends Page Mode and prints the contents
//! of the Page Mode buffer.
//!

use crate::{command::*, constants::*, context::*};

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {
    fn get_device_command(
        &self,
        _command: &Command,
        context: &Context,
    ) -> Option<Vec<DeviceCommand>> {
        return if context.page_mode.enabled {
            Some(vec![DeviceCommand::PrintPageMode])
        } else {
            None
        };
    }
}

pub fn new() -> Command {
    Command::new(
        "Print Contents of Page Mode",
        vec![ESC, FF],
        CommandType::Control,
        DataType::Custom,
        Box::new(Handler {}),
    )
}
