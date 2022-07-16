use std::sync::Arc;

use crate::parser::*;

struct Handler{
  width: usize,
  height: usize,
  capacity: usize,
  accept_data: bool
}

impl CommandHandler for Handler {
  fn push(&mut self, data: &mut Vec<u8>, byte: u8) -> bool{ 
    let data_len = data.len();

    if !self.accept_data {
      if data_len <= 4 { data.push(byte); }
      let _ = *data.get(0).unwrap() as usize; //Not sure what this is
      let w1 = *data.get(1).unwrap() as usize;
      let w2 = *data.get(2).unwrap() as usize;
      let h1 = *data.get(3).unwrap() as usize;
      let h2 = byte as usize;

      self.width = w1 + w2 * 256;
      self.height = h1 + h2 * 256;
      self.capacity = self.width * self.height;

      data.clear();

      return true;
    }

    if data_len >= self.capacity { return false }
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
    Arc::new(Mutex::new(Handler{
        width: 0,
        height: 0,
        capacity: 0,
        accept_data: false
    }))
  )
}