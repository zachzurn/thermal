use crate::{command::*, constants::*, context::*};

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {
    fn get_device_command(
        &self,
        command: &Command,
        context: &Context,
    ) -> Option<Vec<DeviceCommand>> {
        let n = *command
            .data
            .get(0)
            .unwrap_or(&context.default.as_ref().unwrap().text.line_spacing);

        let k = *command
            .data
            .get(1)
            .unwrap_or(&context.default.as_ref().unwrap().text.line_spacing);

        Some(vec![DeviceCommand::ChangeTabs(n, k)])
    }
}

pub fn new() -> Command {
    Command::new(
        "Set Tab Len",
        vec![ESC, 'D' as u8],
        CommandType::TextStyle,
        DataType::Double,
        Box::new(Handler {}),
    )
}
