use crate::{command::*, constants::*, context::*, graphics::*};

#[derive(Clone)]
struct Handler {
    width: u32,
    height: u32,
    capacity: u32,
    size: u32,
    accept_data: bool,
}

impl CommandHandler for Handler {
    fn get_graphics(&self, command: &Command, _context: &Context) -> Option<GraphicsCommand> {
        Some(GraphicsCommand::Image(Image {
            pixels: command.data.clone(),
            width: self.width,
            height: self.height,
            pixel_type: PixelType::Monochrome(0),
            stretch: (1, 1),
        }))
    }
    fn push(&mut self, data: &mut Vec<u8>, byte: u8) -> bool {
        let data_len = data.len();

        if self.accept_data {
            if self.size >= self.capacity { return false; }

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
            self.capacity = self.width * 3;
            self.height = 24
        } else {
            self.capacity = self.width;
            self.height = 8;
        }

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
            size: 0,
            accept_data: false,
        }),
    )
}
