use crate::parser::*;

#[derive(Clone)]
enum BarcodeType {
  A,
  B,
  Unknown
}

#[derive(Clone)]
struct BarcodeHandler{
  kind: BarcodeType,
  capacity: u8,
  has_capacity: bool,
  accept_data: bool
}

impl CommandHandler for BarcodeHandler {
  fn push(&mut self, data: &mut Vec<u8>, byte: u8) -> bool{ 
    let data_len = data.len();

    //Gather metadata
    if !self.accept_data {
      if byte <= 6 || byte == 41 { self.kind = BarcodeType::A; }
      else if byte >= 65 && byte <= 78 { self.kind = BarcodeType::B; } 
      else { self.kind = BarcodeType::Unknown }
      self.accept_data = true;
      
      return true;
    }

    match self.kind {
      //Barcode a data is nul terminated
      BarcodeType::A => {
        if byte == 0 || byte == LF { return false }
        data.push(byte);
        return true;
      },
      BarcodeType::B => {
        if !self.has_capacity {
          self.capacity = byte;
          return true;
        } else if data_len < self.capacity as usize {
          data.push(byte);
          return true;
        }
        return false;
      },
      BarcodeType::Unknown => return false,
    }
  }
}

pub fn new() -> Command {
  Command::new(
    "Barcode",
    vec![GS, 'k' as u8], 
    CommandType::Graphics,
    DataType::Custom,
    Box::new(BarcodeHandler{
      kind: BarcodeType::Unknown,
      capacity: 0,
      has_capacity: false,
      accept_data: false
    })
  )
}