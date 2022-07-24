use crate::{command::*, constants::*, context::*};

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {
    fn get_device_command(&self, _command: &Command, _context: &Context) -> Option<Vec<DeviceCommand>> {
        Some(vec![DeviceCommand::Initialize])
    }
}

pub fn new() -> Command {
    Command::new(
        "Initialize",
        vec![ESC, '@' as u8],
        CommandType::Initialize,
        DataType::Empty,
        Box::new(Handler {}),
    )
}
