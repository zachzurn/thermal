use crate::{command::*, constants::*, context::*, graphics::*};

#[derive(Clone)]
struct Handler {
    width: u32,
    height: u32,
    capacity: u32,
    col_encoded: bool,
    size: u32,
    accept_data: bool,
    stretch: (u8, u8),
}

impl CommandHandler for Handler {
    fn get_graphics(&self, command: &Command, context: &Context) -> Option<GraphicsCommand> {
        let color = context.graphics.render_colors.color_1;

        if self.col_encoded {
            println!("col_encoded={:?}", self.col_encoded);
            Some(GraphicsCommand::image_from_column_bytes_single_color(
                self.width,
                self.height,
                self.stretch,
                &color,
                ImageFlow::Inline,
                &command.data,
            ))
        } else {
            Some(GraphicsCommand::image_from_raster_bytes_single_color(
                self.width,
                self.height,
                self.stretch,
                &color,
                ImageFlow::Inline,
                &command.data,
                false,
            ))
        }
    }
    fn push(&mut self, data: &mut Vec<u8>, byte: u8) -> bool {
        let data_len = data.len();

        if self.accept_data {
            if self.size >= self.capacity {
                return false;
            }

            data.push(byte);

            self.size += 1;
            return true;
        }

        //Create metadata
        if data_len < 2 {
            data.push(byte);
            return true;
        }

        let m = *data.get(0).unwrap() as u32;
        let p1 = *data.get(1).unwrap() as u32;
        let p2 = byte as u32;

        self.width = p1 + p2 * 256;

        if m == 32 || m == 33 {
            //24 dot mode (m = 32, 33)
            self.height = 24;
            self.capacity = (self.width * self.height) / 8;
            self.col_encoded = true;
        } else {
            //8 dot mode (m = 0, 1)
            self.height = 8;
            self.capacity = (self.width * self.height) / 8;
        }

        //Image is single density, which oddly enough needs to
        //have its pixels stretched by 2 on the w and 3 on the h
        if m == 0 || m == 32 {
            self.stretch = (2, 3);
        }

        //After this, we accept data until the capacity is met
        self.accept_data = true;
        data.clear();
        true
    }
}

pub fn new() -> Command {
    Command::new(
        "Bit Image",
        vec![ESC, '*' as u8],
        CommandType::Graphics,
        DataType::Custom,
        Box::new(Handler {
            width: 0,
            height: 0,
            capacity: 0,
            col_encoded: true,
            size: 0,
            accept_data: false,
            stretch: (1, 1),
        }),
    )
}
