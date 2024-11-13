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
        let mut layers = vec![];

        if context.graphics.buffer_graphics.len() > 0 {
            for g in context.graphics.buffer_graphics.iter() {
                match g {
                    GraphicsCommand::Error(_) => return Some(g.clone()),
                    GraphicsCommand::Image(img) => layers.push(img.clone()),
                    _ => {}
                }
            }

            if layers.is_empty() {
                return None;
            }

            if let Ok(merged) = merge_image_layers(&layers) {
                return Some(GraphicsCommand::Image(merged));
            } else {
                return Some(GraphicsCommand::Error(
                    "Could not merge image layers".to_string(),
                ));
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
