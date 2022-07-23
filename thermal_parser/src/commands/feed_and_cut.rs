use crate::{command::*, constants::*, context::*};

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {
    fn get_device_command(&self, _command: &Command, _context: &Context) -> Option<DeviceCommand> {
        let m = *command.data.get(0).unwrap_or(&0u8);
        let n = *command.data.get(1).unwrap_or(&0u8);

        return match m {
            0 | 48 => {
                Some(DeviceCommand::FullCut)
            }
            1 | 49 => {
                Some(DeviceCommand::PartialCut)
            }
            65 | 97 | 103 => {
                Some(DeviceCommand::FullCutFeed(n))
            }
            66 | 98 | 104 => {
                Some(DeviceCommand::PartialCutFeed(n))
            }
            _ => None
        }
    }

    fn push(&mut self, data: &mut Vec<u8>, byte: u8) -> bool {
        if data.len() == 0 { data.push(byte) };
        if data.len() == 1 {
            match data.get(0).unwrap() {
                0u8 | 48u8 | 1u8 | 49u8 => return false,
                _default => data.push(byte)
            }
        };
        false
    }
}

pub fn new() -> Command {
    Command::new(
        "Feed and Cut",
        vec![GS, 'V' as u8],
        CommandType::Control,
        DataType::Custom, //push is implemented in the CommandHandler for Custom types
        Box::new(Handler {}),
    )
}
