//!
//! This command is used to set the y position in page mode.
//!
//! Page mode has the concept of print direction, thus we have
//! to set the x or y based on what the print direction is set to.
//!
//! The position cannot exceed the width or height that is set
//! in page mode.
//!
//! This command is only applicable when page mode is enabled.
//!
use crate::command::{Command, CommandHandler, CommandType, DataType};
use crate::constants::GS;
use crate::context::{Context, PrintDirection};

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {
    fn apply_context(&self, command: &Command, context: &mut Context) {
        if context.page_mode.enabled {
            let nl = *command.data.get(0).unwrap_or(&0u8);
            let nh = *command.data.get(1).unwrap_or(&0u8);

            let pos = (nl as u16 + nh as u16 * 256) as usize;

            match context.page_mode.dir {
                PrintDirection::TopLeft2Right | PrintDirection::BottomRight2Left => {
                    println!("Setting Page Mode y{} to {} pmh{}",pos, usize::min(pos, context.page_mode.h),context.page_mode.h);
                    context.page_mode.y = usize::min(pos, context.page_mode.h)
                }
                PrintDirection::TopRight2Bottom | PrintDirection::BottomLeft2Top => {
                    println!("Setting Page Mode x{} to {} pmw{}", pos, usize::min(pos, context.page_mode.w),context.page_mode.w);
                    context.page_mode.x = usize::min(pos, context.page_mode.w)
                }
            }
        }
    }
}

pub fn new() -> Command {
    Command::new(
        "Set Absolute Vertical Print POS", //should be JQuery Command :)
        vec![GS, '$' as u8],
        CommandType::Context,
        DataType::Double,
        Box::new(Handler {}),
    )
}
