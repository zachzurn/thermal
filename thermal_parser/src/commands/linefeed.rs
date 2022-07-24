use crate::{command::*, constants::*, context::*};

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {
    fn get_device_command(&self, _command: &Command, _context: &Context) -> Option<Vec<DeviceCommand>> {
        Some(vec![DeviceCommand::FeedLine(1)])
    }
}

pub fn new() -> Command {
    Command::new(
        "Line Feed",
        vec![LF],
        CommandType::Text,
        DataType::Empty,
        Box::new(Handler {}),
    )
}
