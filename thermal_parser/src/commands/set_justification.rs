use crate::{command::*, constants::*, context::*};
use crate::command::DeviceCommand::Justify;

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {
    fn get_device_command(&self, command: &Command, _context: &Context) -> Option<Vec<DeviceCommand>> {
        let n = *command.data.get(0).unwrap_or(&0u8);
        
        Some(vec![Justify(
            match n {
                0 | 48 => TextJustify::Left,
                1 | 49 => TextJustify::Center,
                2 | 50 => TextJustify::Right,
                _ => TextJustify::Left,
            }    
        )])
    }
}

pub fn new() -> Command {
    Command::new(
        "Set Text Justification",
        vec![ESC, 'a' as u8],
        CommandType::Control,
        DataType::Single,
        Box::new(Handler {}),
    )
}
