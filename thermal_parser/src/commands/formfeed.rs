use crate::command::DeviceCommand::{EndPageMode, EndPrint};
use crate::{command::*, constants::*, context::*};

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {
    //! Ends page mode if it is enabled. Otherwise, ends the print job.
    fn get_device_command(
        &self,
        _command: &Command,
        context: &Context,
    ) -> Option<Vec<DeviceCommand>> {
        if context.page_mode.enabled {
            return Some(vec![EndPageMode(true)]);
        }
        Some(vec![EndPrint])
    }
}

pub fn new() -> Command {
    Command::new(
        "Form Feed",
        vec![FF],
        CommandType::Control,
        DataType::Empty,
        Box::new(Handler {}),
    )
}
