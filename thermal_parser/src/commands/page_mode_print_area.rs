//! Page Mode Area
//!
//! This command sets or grows the logical page mode area which
//! is a virtual area where graphics and text can be rendered to.
//!
//! Page mode has a logical area (Renderable space) and a
//! page area (The combined area) after numerous areas have been set.
//!
//! Page area is as wide and tall as the widest/tallest areas that were set
//! during a page mode session. Width and height of the page area grows
//! to accommodate areas as they are set. This includes x and y dimensions.
//!
//! To illustrate how this works.
//!
//! Render area starts at 0 width 0 height when page mode is first set
//!
//! Area is set to x = 5, y = 0, w = 200, y = 200
//! Page size becomes 205 x 200
//!
//! Area is set to x = 30, y = 40, w = 300, h = 100
//! Page size stays 300 x 200
//!
//! Area is set to x = 300, y = 0, w = 300, h = 100
//! Page size becomes 600 x 200
//!
//! x and y is the starting x and y when graphics
//! commands are rendered to the render area.
//!
//! x and y have different cardinal directions based on
//! the print direction that is set.
//!
//! Dimension that go beyond the physical print area
//! (max receipt width) will be clipped to the max receipt width.
//!
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

            //Only logical elements should be set here.
            // All other area fields in the page_mode struct
            // are reserved For the rendering context
            context.page_mode.logical_area = RenderArea {
                x: horizontal_logical_origin as u32,
                y: vertical_logical_origin as u32,
                w: print_area_width as u32,
                h: print_area_height as u32,
            }
        }
    }

    fn get_device_command(
        &self,
        _command: &Command,
        _context: &Context,
    ) -> Option<Vec<DeviceCommand>> {
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
