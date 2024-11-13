/// Motion units in this library are divisors
/// If the dots_per_inch is 203
///
/// dpi   val  =  mu
/// 203 / 203  =  1 <-- motion units are divided by 1
/// 203 / 101 =   2 <-- motion units are divided by 2
///
use crate::{command::*, constants::*, context::*};

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {
    fn apply_context(&self, command: &Command, context: &mut Context) {
        let x = *command.data.get(0).unwrap_or(&0u8);
        let y = *command.data.get(1).unwrap_or(&0u8);

        if x == 0 || y == 0 {
            return;
        }

        let h_motion_unit = context.graphics.dots_per_inch / x as u16;
        let v_motion_unit = context.graphics.dots_per_inch / y as u16;

        if x > 0 {
            context.graphics.h_motion_unit = h_motion_unit as u8;
        } else {
            if let Some(default_context) = &context.default {
                context.graphics.h_motion_unit = default_context.graphics.h_motion_unit;
            }
        }

        if y > 0 {
            context.graphics.v_motion_unit = v_motion_unit as u8;
        } else {
            if let Some(default_context) = &context.default {
                context.graphics.v_motion_unit = default_context.graphics.v_motion_unit;
            }
        }
    }
}

pub fn new() -> Command {
    Command::new(
        "Set Vertical and Horizontal Motion Units",
        vec![GS, 'P' as u8],
        CommandType::Context,
        DataType::Double,
        Box::new(Handler {}),
    )
}
