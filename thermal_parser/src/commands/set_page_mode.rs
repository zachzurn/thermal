use crate::command::DeviceCommand::BeginPageMode;
use crate::context::Context;
use crate::{command::*, constants::*};

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {
    fn get_device_command(
        &self,
        _command: &Command,
        _context: &Context,
    ) -> Option<Vec<DeviceCommand>> {
        Some(vec![BeginPageMode])
    }
}

pub fn new() -> Command {
    Command::new(
        "Set page mode",
        vec![ESC, 'L' as u8],
        CommandType::ContextControl,
        DataType::Empty,
        Box::new(Handler {}),
    )
}
