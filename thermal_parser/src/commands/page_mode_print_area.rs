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
use crate::util::parse_u16;
use crate::{command::*, constants::*, context::*};

#[derive(Clone)]
struct Handler;

fn calculate_page_area(command: &Command) -> RenderArea {
    let x = parse_u16(&command.data, 0);
    let y = parse_u16(&command.data, 2);
    let w = parse_u16(&command.data, 4);
    let h = parse_u16(&command.data, 6);

    RenderArea {
        x: x as u32,
        y: y as u32,
        w: w as u32,
        h: h as u32,
    }
}

impl CommandHandler for Handler {
    fn apply_context(&self, command: &Command, context: &mut Context) {
        if context.page_mode.enabled {
            //Only logical elements should be set here.
            // All other area fields in the page_mode struct
            // are reserved For the rendering context
            context.set_page_area(calculate_page_area(command))
        }
    }

    fn get_device_command(
        &self,
        _command: &Command,
        _context: &Context,
    ) -> Option<Vec<DeviceCommand>> {
        Some(vec![DeviceCommand::ChangePageArea])
    }

    fn debug(&self, command: &Command, _context: &Context) -> String {
        format!("Set Print Area --> {:?}", calculate_page_area(command))
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
