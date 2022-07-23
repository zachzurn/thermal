use crate::{command::*, constants::*};
use crate::context::Context;
use crate::graphics::GraphicsCommand;

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {
    fn get_device_command(&self, _command: &Command, _context: &Context) -> Option<DeviceCommand> {
        Some(DeviceCommand::Cancel)
    }
}

pub fn new() -> Command {
    Command::new(
        "Cancel",
        vec![CAN, '@' as u8],
        CommandType::Control,
        DataType::Empty,
        Box::new(Handler {}),
    )
}
