use crate::{command::*, constants::*, context::*};

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {
    fn apply_context(&self, command: &Command, context: &mut Context) {
        let xL = *command.data.get(0).unwrap_or(&0u8);
        let xH = *command.data.get(1).unwrap_or(&0u8);
        let yL = *command.data.get(2).unwrap_or(&0u8);
        let yH = *command.data.get(3).unwrap_or(&0u8);
        let dxL = *command.data.get(4).unwrap_or(&0u8);
        let dxH = *command.data.get(5).unwrap_or(&0u8);
        let dyL = *command.data.get(6).unwrap_or(&0u8);
        let dyH = *command.data.get(7).unwrap_or(&0u8);

        let horizontal_logical_origin = (u16::from(xL) + u16::from(xH) * 256) * context.graphics.h_motion_unit as u16;
        let vertical_logical_origin = (u16::from(yL) + u16::from(yH) * 256) * context.graphics.v_motion_unit as u16;

        // Calculate print area dimensions
        let print_area_width = (u16::from(dxL) + u16::from(dxH) * 256) * context.graphics.h_motion_unit as u16;
        let print_area_height = (u16::from(dyL) + u16::from(dyH) * 256) * context.graphics.v_motion_unit as u16;

        // Adjustments for exceeding printable area
        let adjusted_width = if horizontal_logical_origin + print_area_width > context.graphics.x as u16{
            context.graphics.x as u16 - horizontal_logical_origin
        } else {
            print_area_width
        };

        let adjusted_height = if vertical_logical_origin + print_area_height > context.graphics.y as u16 {
            context.graphics.y as u16 - vertical_logical_origin
        } else {
            print_area_height
        };

        context.graphics.y = adjusted_height as usize;
        context.graphics.x = adjusted_width as usize;

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
