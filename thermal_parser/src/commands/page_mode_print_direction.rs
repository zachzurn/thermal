//!
//! This command sets the print direction for Page Mode.
//!
//! Page Mode can be set to 1 of 4 print directions.
//!
//! * Top Left to right
//! * Bottom Left to top
//! * Top Right to Bottom
//! * Bottom Right to Left
//!

use crate::{command::*, constants::*, context::*};

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {
    fn apply_context(&self, command: &Command, context: &mut Context) {
        if context.page_mode.enabled {
            let dir = *command.data.get(0).unwrap_or(&0u8);

            let direction = match dir {
                0 => PrintDirection::TopLeft2Right,
                48 => PrintDirection::TopLeft2Right,

                1 => PrintDirection::BottomLeft2Top,
                49 => PrintDirection::BottomLeft2Top,

                2 => PrintDirection::BottomRight2Left,
                50 => PrintDirection::BottomRight2Left,

                3 => PrintDirection::TopRight2Bottom,
                51 => PrintDirection::TopRight2Bottom,

                _ => PrintDirection::TopLeft2Right,
            };

            context.page_mode.dir = direction;
            context.page_mode.x = context.page_mode.logical_x;
            context.page_mode.y = context.page_mode.logical_y;
        }
    }

    fn get_device_command(
        &self,
        _command: &Command,
        _context: &Context,
    ) -> Option<Vec<DeviceCommand>> {
        Some(vec![DeviceCommand::ChangePageModeDirection])
    }
}

pub fn new() -> Command {
    Command::new(
        "Set Page Mode Print Direction",
        vec![ESC, 'T' as u8],
        CommandType::ContextControl,
        DataType::Single,
        Box::new(Handler {}),
    )
}
