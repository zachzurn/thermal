use crate::util::parse_u16;
use crate::{command::*, context::*, graphics::*};

#[derive(Clone)]
pub struct Handler;

impl CommandHandler for Handler {
    fn apply_context(&self, command: &Command, context: &mut Context) {
        let _a = command.data.get(0).unwrap();
        let bx = command.data.get(1).unwrap();
        let by = command.data.get(2).unwrap();
        let c = command.data.get(3).unwrap();
        let width = parse_u16(&command.data, 4) as u32;
        let height = parse_u16(&command.data, 6) as u32;
        let stretch = (*bx, *by);

        let graphics = GraphicsCommand::image_from_raster_bytes_single_color(
            width,
            height,
            stretch,
            context.graphics.render_colors.color_for_number(*c),
            ImageFlow::Block,
            &command.data[8..],
            true,
        );

        match graphics {
            GraphicsCommand::Image(mut image) => {
                image.flow = ImageFlow::Block;
                context.graphics.buffer_graphics.push(image);
            }
            GraphicsCommand::Error(error) => {
                println!("{:?}", error);
            }
            _ => {
                println!("Unexpected graphics command for image");
            }
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
