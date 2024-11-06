use crate::{command::*, context::*, graphics::*};

#[derive(Clone)]
pub struct Handler;

impl CommandHandler for Handler {
    fn apply_context(&self, command: &Command, context: &mut Context) {
        if let Some(mut img) =
            Image::from_raster_data(&command.data, &context.graphics.render_colors)
        {
            img.flow = ImageFlow::Block;
            context.graphics.buffer_graphics.push(img)
        }
    }

    fn debug(&self, command: &Command, context: &Context) -> String {
        if let Some(img) = Image::from_raster_data(&command.data, &context.graphics.render_colors) {
            format!(
                "Graphics Raster format {{ {} w: {} h: {} pixels: {} }}",
                img.w,
                img.h,
                img.pixels.len()
            )
        } else {
            "Graphics raster format failed to create image".to_string()
        }
    }
}

//Deletes the Download graphics data defined by the key codes (kc1 and kc2).
pub fn new() -> Command {
    Command::new(
        "Store Print Buffer Graphics Raster Format",
        vec![112],
        CommandType::Context,
        DataType::Subcommand,
        Box::new(Handler),
    )
}
