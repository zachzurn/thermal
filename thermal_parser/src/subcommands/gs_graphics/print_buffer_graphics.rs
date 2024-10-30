use crate::{command::*, context::*, graphics::*};

#[derive(Clone)]
pub struct Handler;

fn merge_images(images: &Vec<Image>) -> Option<Image> {
    if images.is_empty() {
        return None;
    }
    if images.len() == 1 {
        return Some(images[0].clone());
    }

    let mut image = images[0].clone();
    let mut height = image.h;

    //For the moment, we only merge if all images have the same width
    for merge_img in images.iter().skip(1) {
        if merge_img.w > image.w {
            return None;
        }

        height += merge_img.h;
        image.pixels.append(&mut merge_img.pixels.clone());
    }

    image.h = height;

    Some(image)
}

impl CommandHandler for Handler {
    fn get_graphics(&self, _command: &Command, context: &Context) -> Option<GraphicsCommand> {
        if context.graphics.buffer_graphics.len() > 0 {
            if let Some(merged) = merge_images(&context.graphics.buffer_graphics) {
                return Some(GraphicsCommand::Image(merged));
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
