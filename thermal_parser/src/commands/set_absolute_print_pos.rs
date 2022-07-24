use crate::{command::*, constants::*, context::*};

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {
    fn get_device_command(&self, command: &Command, _context: &Context) -> Option<Vec<DeviceCommand>> {
        let nl = *command.data.get(0).unwrap_or(&0u8);
        let nh = *command.data.get(0).unwrap_or(&0u8);
        Some(vec![DeviceCommand::MoveX(nl as u16 + nh as u16 * 256)])
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
