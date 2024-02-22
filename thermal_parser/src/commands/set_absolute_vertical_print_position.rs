use crate::command::{Command, CommandHandler, CommandType, DataType, DeviceCommand};
use crate::constants::{GS};
use crate::context::Context;

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {
    fn get_device_command(
        &self,
        command: &Command,
        _context: &Context,
    ) -> Option<Vec<DeviceCommand>> {
        if _context.is_page_mode {
            let nl = *command.data.get(0).unwrap_or(&0u8);
            let nh = *command.data.get(1).unwrap_or(&0u8);
            println!("xL: {}, xH: {}",
                     nl, nh);
            Some(vec![DeviceCommand::MoveX(nl as u16 + nh as u16 * 256)])
        }
        None

    }
}

pub fn new() -> Command {
    Command::new(
        "Set Absolute Vertical Print POS", //should be JQuery Command :)
        vec![GS, '$' as u8],
        CommandType::Control,
        DataType::Double,
        Box::new(Handler {}),
    )
}
