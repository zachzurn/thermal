use crate::{command::*, constants::*, context::*, graphics, graphics::*};

#[derive(Clone)]
struct Handler {
    width: u32,
    height: u32,
    capacity: u32,
    scaling: u8,
    accept_data: bool,
}

impl CommandHandler for Handler {
    fn get_graphics(&self, command: &Command, _context: &Context) -> Option<GraphicsCommand> {
        let stretch = match self.scaling {
            0 | 48 => (1, 1),
            1 | 49 => (2, 1),
            2 | 50 => (1, 2),
            3 | 52 => (2, 2),
            _ => (1, 1),
        };

        let mut pixels = command.data.clone();

        pack_color_levels(&mut pixels, 1);

        if stretch.0 > 1 || stretch.1 > 1 {
            let scaled = graphics::scale_pixels(
                &pixels,
                self.width as u32,
                self.height as u32,
                stretch.0,
                stretch.1,
            );

            let mut img = Image {
                pixels: scaled.2,
                x: 0,
                y: 0,
                w: scaled.0,
                h: scaled.1,
                stretch,
                flow: ImageFlow::None,
                upside_down: false,
            };

            img.unpack_bit_encoding();

            Some(GraphicsCommand::Image(img))
        } else {
            let mut img = Image {
                pixels,
                x: 0,
                y: 0,
                w: self.width,
                h: self.height,
                stretch,
                flow: ImageFlow::None,
                upside_down: false,
            };

            img.unpack_bit_encoding();

            Some(GraphicsCommand::Image(img))
        }
    }
    fn push(&mut self, data: &mut Vec<u8>, byte: u8) -> bool {
        let data_len = data.len();

        if !self.accept_data {
            if data_len < 4 {
                data.push(byte);
                return true;
            }
            self.scaling = *data.get(0).unwrap();
            let xl = *data.get(1).unwrap() as u32;
            let xh = *data.get(2).unwrap() as u32;
            let yl = *data.get(3).unwrap() as u32;
            let yh = byte as u32;

            self.width = xl + xh * 256;
            self.height = yl + yh * 256;
            self.capacity = self.width * self.height;
            self.width = self.width * 8;

            data.clear();

            self.accept_data = true;
            return true;
        }

        if data_len >= self.capacity as usize {
            return false;
        }
        data.push(byte);
        true
    }
}

pub fn new() -> Command {
    Command::new(
        "Raster Bit Image",
        vec![GS, 'v' as u8, '0' as u8],
        CommandType::Graphics,
        DataType::Custom,
        Box::new(Handler {
            width: 0,
            height: 0,
            capacity: 0,
            scaling: 0,
            accept_data: false,
        }),
    )
}
