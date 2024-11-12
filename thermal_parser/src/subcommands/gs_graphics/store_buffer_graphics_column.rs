use crate::util::parse_u16;
use crate::{command::*, context::*, graphics::*};

#[derive(Clone)]
pub struct Handler;

impl CommandHandler for Handler {
    fn apply_context(&self, command: &Command, context: &mut Context) {
        if command.data.len() < 4 {
            context.graphics.buffer_graphics.push(GraphicsCommand::Error("Not enough parameters for buffer graphics".to_owned()));
            return;
        }

        let _a = command.data.get(0).unwrap();
        let bx = command.data.get(1).unwrap();
        let by = command.data.get(2).unwrap();
        let c = command.data.get(3).unwrap();
        let width = parse_u16(&command.data, 4) as u32;
        let height = parse_u16(&command.data, 6) as u32;

        let stretch = (*bx, *by);

        let graphics = GraphicsCommand::image_from_column_bytes_single_color(
            width,
            height,
            stretch,
            context.graphics.render_colors.color_for_number(*c),
            ImageFlow::Block,
            &command.data[8..],
        );
        
        context.graphics.buffer_graphics.push(graphics);
    }
}

pub fn new() -> Command {
    Command::new(
        "Store Print Buffer Graphics Table Format",
        vec![113],
        CommandType::Context,
        DataType::Subcommand,
        Box::new(Handler),
    )
}
