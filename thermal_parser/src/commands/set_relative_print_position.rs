use crate::context::Context;
use crate::{command::*, constants::*};

#[derive(Clone)]
struct Handler;

/// When Standard mode is selected the horizontal
/// x is offset by the value, which can be positive or negative
///
/// When Page mode is selected, the horizontal or vertical
/// motion unit is used for the print direction set by ESC T.
impl CommandHandler for Handler {
    fn apply_context(&self, command: &Command, context: &mut Context) {
        if context.page_mode.enabled {
            context.offset_x_relative(get_pos(&command.data));
        }
    }

    fn debug(&self, command: &Command, _context: &Context) -> String {
        format!("{} --> {}", &command.name, get_pos(&command.data))
    }
}

fn get_pos(data: &Vec<u8>) -> i16 {
    let nl = data.get(0).unwrap_or(&0u8);
    let nh = data.get(1).unwrap_or(&0u8);

    let large = *nl as u32 + (*nh as u32 * 256);

    if large > i16::MAX as u32 {
        return 0 - (u16::MAX as u32 - large) as i16;
    }

    large as i16
}

pub fn new() -> Command {
    Command::new(
        "Set Relative Horizontal Position",
        vec![GS, '\\' as u8],
        CommandType::Context,
        DataType::Double,
        Box::new(Handler {}),
    )
}
