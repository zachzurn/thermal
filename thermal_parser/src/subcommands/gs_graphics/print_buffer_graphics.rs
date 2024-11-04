/// This command prints everything that was added into the print buffer
///
/// There is some uncertainty around the actual behavior of this command.
///
/// Our best guess is to merge pixels by darkening the pixels.
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

    //For the moment, we only merge if all images have the same width and height
    for merge_img in images.iter().skip(1) {
        if merge_img.w > image.w || merge_img.h > image.h {
            println!("Ignored merge image");
            return None;
        }

        //Copy any pixels that are 255 into the image in place
        for (i, b) in merge_img.pixels.iter().enumerate() {
            image.pixels[i] = image.pixels[i].saturating_add(*b);
        }
    }

    Some(image)
}

impl CommandHandler for Handler {
    fn get_graphics(&self, _command: &Command, context: &Context) -> Option<GraphicsCommand> {
        if context.graphics.buffer_graphics.len() > 0 {
            for buffer_graphic in context.graphics.buffer_graphics.iter() {}

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
