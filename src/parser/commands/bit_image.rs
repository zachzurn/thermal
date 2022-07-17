use crate::parser::*;
use crate::parser::graphics::*;

#[derive(Clone)]
struct Handler{
  width: u32,
  height: u32,
  capacity: u32,
  size: u32,
  is_bit_image: bool,
  buffer: u8,
  accept_data: bool
}

impl CommandHandler for Handler {
  fn get_graphics(&self, command: &Command) -> Option<GraphicsCommand> {
    let pixtype = if self.is_bit_image { PixelType::Bit } else { PixelType::Byte };  
    Some(GraphicsCommand::Image(Image{
        pixels: command.data.clone(),
        width: self.width,
        height: self.height,
        pixel_type: pixtype,
        storage_id: None,
    }))
  }
  fn push(&mut self, data: &mut Vec<u8>, byte: u8) -> bool{ 
    let data_len = data.len();

    if self.accept_data {
      if self.size >= self.capacity { return false; }
      
      if self.is_bit_image {
        //For Bit Images (only black or white pixels) we can store them in a compressed format
        //Here we are storing compressed image data since the bytes only ever contain 0 or 1
        let bit_index = self.size % 8;
        
                                             //set the nth bit to on
        if byte > 0 { self.buffer = (1 << bit_index) | self.buffer }
        
        if bit_index == 7 { 
          data.push(self.buffer); 
          self.buffer = 0;
        }
      } else {
        data.push(byte);
      }
      
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
    Box::new(Handler{
      width: 0,
      height: 0,
      capacity: 0,
      size: 0,
      is_bit_image: false,
      buffer: 0,
      accept_data: false
    })
  )
}