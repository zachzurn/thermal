/// This command prints everything that was added into the print buffer
///
/// There is some uncertainty around the actual behavior of this command.
///
/// Our best guess is to merge pixels by darkening the pixels.
use crate::{command::*, context::*, graphics::*};

#[derive(Clone)]
pub struct Handler;

impl CommandHandler for Handler {
    fn get_graphics(&self, _command: &Command, context: &Context) -> Option<GraphicsCommand> {
        if context.graphics.buffer_graphics.len() > 0 {
            for buffer_graphic in context.graphics.buffer_graphics.iter() {}

            if let Ok(merged) = merge_image_layers(&context.graphics.buffer_graphics) {
                return Some(GraphicsCommand::Image(merged));
            } else {
                println!("Failed to merge image layers");
            }
        }
        None
    }

    fn get_device_command(
        &self,
        _command: &Command,
        _context: &Context,
    ) -> Option<Vec<DeviceCommand>> {
        Some(vec![DeviceCommand::ClearBufferGraphics])
    }
}

pub fn new() -> Command {
    Command::new(
        "Print Buffer Graphics",
        vec![2, 50],
        CommandType::Graphics,
        DataType::Subcommand,
        Box::new(Handler),
    )
}
