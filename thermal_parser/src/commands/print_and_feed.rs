use crate::{command::*, constants::*};
use crate::context::Context;
use crate::graphics::GraphicsCommand;

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {
    fn get_device_command(&self, _command: &Command, _context: &Context) -> Option<DeviceCommand> {
        let n = *command.data.get(0).unwrap_or(&0u8);
        Some(DeviceCommand::PrintAndFeed(n as i16))
    }
}

pub fn new() -> Command {
    Command::new(
        "Print and Feed",
        vec![ESC, 'J' as u8],
        CommandType::Control,
        DataType::Single,
        Box::new(Handler {}),
    )
}
