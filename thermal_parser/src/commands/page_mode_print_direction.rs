use crate::{command::*, constants::*, context::*};

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {
    fn apply_context(&self, command: &Command, context: &mut Context) {
        if context.page_mode.enabled {
            let dir = *command.data.get(0).unwrap_or(&0u8);

            let direction = match dir {
                0 => PrintDirection::Left2Right,
                48 => PrintDirection::Left2Right,

                1 => PrintDirection::Bottom2Top,
                49 => PrintDirection::Bottom2Top,

                2 => PrintDirection::Right2Left,
                50 => PrintDirection::Right2Left,

                3 => PrintDirection::Top2Bottom,
                51 => PrintDirection::Top2Bottom,

                _ => PrintDirection::Left2Right,
            };

            context.page_mode.dir = direction;
            context.page_mode.x = 0;
            context.page_mode.y = 0;
        }
    }

    fn get_device_command(&self, _command: &Command, _context: &Context) -> Option<Vec<DeviceCommand>> {
        Some(vec![DeviceCommand::ChangePageModeDirection])
    }
}

pub fn new() -> Command {
    Command::new(
        "Set Page Mode Print Direction",
        vec![ESC, 'T' as u8],
        CommandType::ContextControl,
        DataType::Single,
        Box::new(Handler {}),
    )
}
