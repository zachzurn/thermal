/// When Standard mode is selected the horizontal
/// x is offset by the value, which can be positive or negative
///
/// When Page mode is selected, the horizontal or vertical
/// motion unit is used for the print direction set by ESC T.
use crate::context::{Context, PrintDirection};
use crate::{command::*, constants::*};

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {
    fn apply_context(&self, command: &Command, context: &mut Context) {
        // Command is only applicable in page mode
        if context.page_mode.enabled {
            context.offset_y_relative(get_pos(&command.data));

            println!("Set relative vert pos for page mode");
            println!(
                "New pos x{} y{}",
                context.page_mode.render_area.x, context.page_mode.render_area.y
            );
        }
    }

    fn debug(&self, command: &Command, _context: &Context) -> String {
        format!("{} --> {}", &command.name, get_pos(&command.data))
    }
}

fn get_pos(data: &Vec<u8>) -> i16 {
    let nl = data.get(0).unwrap_or(&0u8);
    let nh = data.get(1).unwrap_or(&0u8);

    let large = *nl as u16 + (*nh as u16 * 256);

    large as i16
}

pub fn new() -> Command {
    Command::new(
        "Set Relative Vertical Position",
        vec![GS, '\\' as u8],
        CommandType::Context,
        DataType::Double,
        Box::new(Handler {}),
    )
}
