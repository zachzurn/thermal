use crate::{command::*, constants::*, context::*};

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {
    fn apply_context(&self, command: &Command, context: &mut Context) {
        let x = *command.data.get(0).unwrap_or(&0u8);
        let y = *command.data.get(1).unwrap_or(&0u8);

        if x > 0 {
            context.graphics.v_motion_unit = 1f32 / x as f32;
        }else {
            context.reset_graphics_v_motion_units();
        }

        if y > 0 {
            context.graphics.h_motion_unit = 1f32 / y as f32;
        } else {
            context.reset_graphics_h_motion_units();
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
