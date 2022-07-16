use std::sync::{ Arc, Mutex };

use crate::parser::CommandHandler;

struct DataHandler{
  is_large: bool,
  arg1: u8,
  arg2: u8,
  capacity: u32, //small is u16 but large is u32 so we hold the larger
  accept_data: bool
}

//maybe can be done better
fn capacity_4(b1: &u8, b2: &u8, b3: &u8, b4: &u8) -> u32 {
  *b1 as u32 + *b2 as u32 * 256 + *b3 as u32 * 65536 + *b4 as u32 * 16777216
}

//maybe can be done better
fn capacity_2(b1: &u8, b2: &u8) -> u32 {
  *b1 as u32 + *b2 as u32 * 256
}

impl CommandHandler for DataHandler {
  fn push(&mut self, data: &mut Vec<u8>, byte: u8) -> bool{ 
    let data_len = data.len();

    if !self.accept_data {
      match self.is_large {
        //Large data metadata
        true => {
          if data_len <= 6 { data.push(byte); return true; }

          self.arg1 = *data.get(4).unwrap();
          self.arg2 = *data.get(5).unwrap();

          self.capacity = capacity_4(data.get(0).unwrap(), data.get(1).unwrap(), data.get(2).unwrap(), data.get(3).unwrap()); //can be done better
          data.clear();
          self.accept_data = true;
        },
        //Small data metadata
        false => {
          if data_len <= 4 { data.push(byte); return true; }

          self.arg1 = *data.get(2).unwrap();
          self.arg2 = *data.get(3).unwrap();
          
          self.capacity = capacity_2(data.get(0).unwrap(), data.get(1).unwrap());
          data.clear();
          self.accept_data = true;
        },
      }
    }

    //Accept data
    if data_len >= self.capacity as usize { return false }
    data.push(byte);
    true    
  }
}

pub fn new(is_large: bool) -> Arc<Mutex<dyn CommandHandler>> {
  Arc::new(Mutex::new(DataHandler{
    is_large,
    arg1: 0,
    arg2: 0,
    capacity: 0,
    accept_data: false,
  }))
}