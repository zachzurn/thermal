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

            context.page_mode.x = horizontal_logical_origin as usize;
            context.page_mode.y = vertical_logical_origin as usize;
            context.page_mode.w = print_area_width as usize;
            context.page_mode.h = print_area_height as usize;
        }
    }

    fn get_device_command(&self, _command: &Command, _context: &Context) -> Option<Vec<DeviceCommand>> {
        Some(vec![DeviceCommand::ChangePageArea])
    }
}

pub fn new() -> Command {
    Command::new(
        "Set Print Area",
        vec![ESC, 'W' as u8],
        CommandType::ContextControl,
        DataType::Octet,
        Box::new(Handler {}),
    )
}
