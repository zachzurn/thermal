use crate::{command::*, constants::*, context::*};

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {
    fn apply_context(&self, command: &Command, context: &mut Context) {
        if context.page_mode.enabled {
            let x_l = *command.data.get(0).unwrap_or(&0u8);
            let x_h = *command.data.get(1).unwrap_or(&0u8);
            let y_l = *command.data.get(2).unwrap_or(&0u8);
            let y_h = *command.data.get(3).unwrap_or(&0u8);
            let dx_l = *command.data.get(4).unwrap_or(&0u8);
            let dx_h = *command.data.get(5).unwrap_or(&0u8);
            let dy_l = *command.data.get(6).unwrap_or(&0u8);
            let dy_h = *command.data.get(7).unwrap_or(&0u8);

            let horizontal_logical_origin =
                (u16::from(x_l) + u16::from(x_h) * 256) * context.graphics.h_motion_unit as u16;
            let vertical_logical_origin =
                (u16::from(y_l) + u16::from(y_h) * 256) * context.graphics.v_motion_unit as u16;

            // Calculate print area dimensions
            let print_area_width =
                (u16::from(dx_l) + u16::from(dx_h) * 256) * context.graphics.h_motion_unit as u16;
            let print_area_height =
                (u16::from(dy_l) + u16::from(dy_h) * 256) * context.graphics.v_motion_unit as u16;

            let adjusted_width = if context.graphics.x as u16 >= horizontal_logical_origin {
                context.graphics.x as u16 - horizontal_logical_origin
            } else {
                0
            };

            let adjusted_height = if context.graphics.y as u16 >= vertical_logical_origin {
                context.graphics.y as u16 - vertical_logical_origin
            } else {
                0
            };

            context.graphics.y = adjusted_height as usize;
            context.graphics.x = adjusted_width as usize;
        }
    }
}

pub fn new() -> Command {
    Command::new(
        "Set Print Area",
        vec![ESC, 'W' as u8],
        CommandType::Context,
        DataType::Octet,
        Box::new(Handler {}),
    )
}
