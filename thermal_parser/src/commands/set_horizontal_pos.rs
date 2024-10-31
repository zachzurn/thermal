///
/// This command is used to set the x position in standard and page mode.
///
/// The position cannot exceed the width or height that is set
///
/// In page mode, the x/y may need to be swapped based on the orientation.
///
use crate::{command::*, constants::*, context::*};

#[derive(Clone)]
struct Handler;

fn get_pos(data: &Vec<u8>) -> u32 {
    let nl = data.get(0).unwrap_or(&0u8);
    let nh = data.get(1).unwrap_or(&0u8);

    (*nl as u16 + *nh as u16 * 256) as u32
}

impl CommandHandler for Handler {
    fn apply_context(&self, command: &Command, context: &mut Context) {
        if context.page_mode.enabled {
            context.set_x(get_pos(&command.data));

            println!("Set horiz pos for page mode");
            println!(
                "New pos x{} y{}",
                context.page_mode.render_area.x, context.page_mode.render_area.y
            );
        } else {
            context.set_x(get_pos(&command.data));
        }
    }

    fn debug(&self, command: &Command, _context: &Context) -> String {
        format!("{} --> {}", &command.name, get_pos(&command.data))
    }
}

pub fn new() -> Command {
    Command::new(
        "Set Absolute Horizontal Position", //should be JQuery Command :)
        vec![ESC, '$' as u8],
        CommandType::Context,
        DataType::Double,
        Box::new(Handler {}),
    )
}
