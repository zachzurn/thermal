use crate::{command::*, constants::*, context::*};

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {
    fn get_device_command(
        &self,
        command: &Command,
        _context: &Context,
    ) -> Option<Vec<DeviceCommand>> {
        let n = *command.data.get(0).unwrap_or(&0u8);
        Some(vec![DeviceCommand::FeedLine(n as i16)])
    }
}

pub fn new() -> Command {
    Command::new(
        "Print and Feed Lines",
        vec![ESC, 'd' as u8],
        CommandType::Control,
        DataType::Single,
        Box::new(Handler {}),
    )
}
