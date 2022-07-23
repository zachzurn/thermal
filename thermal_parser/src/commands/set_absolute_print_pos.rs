use crate::{command::*, constants::*};
use crate::command::DeviceCommand::MoveX;
use crate::context::Context;
use crate::graphics::GraphicsCommand;

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {
    fn get_device_command(&self, _command: &Command, _context: &Context) -> Option<DeviceCommand> {
        let nl = *command.data.get(0).unwrap_or(&0u8);
        let nh = *command.data.get(0).unwrap_or(&0u8);
        Some(MoveX(nl + nh * 256))
    }
}

pub fn new() -> Command {
    Command::new(
        "Set Absolute Print POS", //should be JQuery Command :)
        vec![ESC, '$' as u8],
        CommandType::Control,
        DataType::Double,
        Box::new(Handler {}),
    )
}
