//!
//! This command sets the print direction for Page Mode.
//!
//! When this command is parsed, then any page area
//! will be applied. It should be sent after setting
//! page area @see: page_mode_print_area.rs
//!
//! Page Mode can be set to 1 of 4 print directions.
//!
//! * Top Left to right
//! Renders from top left of the page
//! to the right (standard direction)
//!
//!   x →
//! y ----------
//! ↓ | →      |
//!   |        |
//!   |        |
//!   ----------
//!  
//!
//! * Bottom Left to top
//! Renders from bottom left of page to the top
//!
//!   ----------
//!   |        |
//!   |        |
//! ↑ | ↑      |
//! x ----------
//!   y →
//!
//! * Top Right to Bottom
//! Renders from top right to bottom
//!
//!        ← y
//! ---------- x
//! |      ↓ | ↓
//! |        |
//! |        |
//! ----------
//!
//! * Bottom Right to Left
//! Renders from bottom left of page to the top
//!         
//! ----------
//! |        |
//! |        |
//! |      ← | ↑
//! ---------- y
//!        ← x

use crate::{command::*, constants::*, context::*};
use std::mem;

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

            mem::swap(
                &mut context.page_mode.direction,
                &mut context.page_mode.previous_direction,
            );
            context.page_mode.direction = direction;
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
