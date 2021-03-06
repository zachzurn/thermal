use crate::{command::*, constants::*, context::*, graphics::*};

#[derive(Clone)]
enum ImageScaling {}

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
            0x01 => (1, 1),
            0x02 => (2, 1),
            0x03 => (1, 2),
            0x04 => (2, 2),
            _ => (1, 1)
        };

        //possibly implement scaling here
        Some(GraphicsCommand::Image(Image {
            pixels: command.data.clone(),
            width: self.width,
            height: self.height,
            pixel_type: PixelType::Monochrome(1),
            stretch,
        }))
    }
    fn push(&mut self, data: &mut Vec<u8>, byte: u8) -> bool {
        let data_len = data.len();

        if !self.accept_data {
            if data_len <= 4 {
                data.push(byte);
                return true;
            }
            self.scaling = *data.get(0).unwrap(); // 0x01 normal 0x02 double width  0x03  double height 0x04 quadruple
            let w1 = *data.get(1).unwrap() as u32;
            let w2 = *data.get(2).unwrap() as u32;
            let h1 = *data.get(3).unwrap() as u32;
            let h2 = byte as u32;

            self.width = w1 + w2 * 256;
            self.height = h1 + h2 * 256;
            self.capacity = self.width * self.height;

            data.clear();

            return true;
        }

        if data_len >= self.capacity as usize { return false; }
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