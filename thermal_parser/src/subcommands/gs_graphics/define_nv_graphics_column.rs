use crate::util::parse_u16;
use crate::{command::*, context::*, graphics::*};

#[derive(Clone)]
pub struct Handler;

impl CommandHandler for Handler {
    fn apply_context(&self, command: &Command, context: &mut Context) {
        if command.data.len() < 8 {
            println!("Missing parameters for command");
        };

        let _a = command.data.get(0).unwrap();
        let kc1 = *command.data.get(1).unwrap();
        let kc2 = *command.data.get(2).unwrap();
        let b = command.data.get(3).unwrap(); //Number of color data

        let width = parse_u16(&command.data, 4) as u32;
        let height = parse_u16(&command.data, 6) as u32;
        let stretch = (1, 1);
        let storage = ImageRefStorage::Disc;
        let image_ref = ImageRef { kc1, kc2, storage };

        let graphics = GraphicsCommand::image_from_raster_bytes_multi_color(
            width,
            height,
            stretch,
            *b,
            &context.graphics.render_colors,
            ImageFlow::Block,
            &command.data[8..],
            true,
        );

        match graphics {
            GraphicsCommand::Image(mut image) => {
                image.flow = ImageFlow::Block;
                context.graphics.stored_graphics.insert(image_ref, image);
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

pub fn new() -> Command {
    Command::new(
        "Define NV Graphics in Column Format",
        vec![68],
        CommandType::Context,
        DataType::Subcommand,
        Box::new(Handler),
    )
}
