use crate::{command::*, constants::*, context::*};

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {
    fn get_device_command(&self, _command: &Command, _context: &Context) -> Option<Vec<DeviceCommand>> {
        Some(vec![DeviceCommand::Initialize])
    }
    fn apply_context(&self, _command: &Command, context: &mut Context) {
        context.reset();
    }
}

pub fn new() -> Command {
    Command::new(
        "Initialize",
        vec![ESC, '@' as u8],
        CommandType::ContextCommand,
        DataType::Empty,
        Box::new(Handler {}),
    )
}
