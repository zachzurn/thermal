use crate::parser::{ CommandHandler, Command};

#[derive(Clone)]
pub enum GraphicsDataType {
  PrintGraphics,
  StoreGraphics,
  StoreColumnFmt,
  Unknown
}

#[derive(Clone)]
struct DataHandler{
  kind: GraphicsDataType,
  is_large: bool,
  m: u8,
  function: u8,
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

impl DataHandler {
  fn detect_kind(&mut self){
    match self.function {
        112 => self.kind = GraphicsDataType::StoreGraphics,
        2 | 50 => self.kind = GraphicsDataType::PrintGraphics,
        113 => self.kind = GraphicsDataType::StoreColumnFmt,
        _default => self.kind = GraphicsDataType::Unknown
    }
  }
  fn parse_meta(&mut self, data: &[u8]){
    let data_len = data.len();

    if data_len == 4{
      self.capacity = capacity_2(data.get(0).unwrap(), data.get(1).unwrap());
      self.capacity -= 2;
      self.m = *data.get(2).unwrap();
      self.function = *data.get(3).unwrap();
      self.detect_kind();
    }

    if data_len == 6 {
      self.capacity = capacity_4(data.get(0).unwrap(), data.get(1).unwrap(), data.get(2).unwrap(), data.get(3).unwrap());
      self.capacity -= 2;
      self.m = *data.get(4).unwrap();
      self.function = *data.get(5).unwrap();
      self.detect_kind();
    }

    self.detect_kind();
    self.accept_data = true;
  }
}

impl CommandHandler for DataHandler {
  fn push(&mut self, data: &mut Vec<u8>, byte: u8) -> bool{ 
    let data_len = data.len();

    if !self.accept_data {
      if self.is_large {
        if data_len < 6 { data.push(byte); return true; }
        self.parse_meta(&data[0..6]);
        data.clear();
      } else {
        if data_len < 4 { data.push(byte); return true; }
        self.parse_meta(&data[0..4]);
        data.clear();
      }
    }

    //Accept data
    if data_len < (self.capacity as usize)  { 
      data.push(byte); 
      return true;
    }
    false
  }
  fn debug(&self, command: &Command) -> String {
    let sub_description: &str;
    match self.kind {
      GraphicsDataType::PrintGraphics => sub_description = "Print Image",
      GraphicsDataType::StoreGraphics => sub_description = "Store Image",
      GraphicsDataType::StoreColumnFmt => sub_description = "Store Column Format",
      GraphicsDataType::Unknown => sub_description = "Unknown"
    }

    format!("{} {} m({}) fun({}) capacity({})", command.name, sub_description, self.m, self.function, self.capacity)
    //format!("{} {} m({}) fun({}) capacity({}) {:02X?}", command.name, sub_description, self.m, self.function, self.capacity, command.data)
  }
}

pub fn new(is_large: bool) -> Box<dyn CommandHandler> {
  Box::new(DataHandler{
    kind: GraphicsDataType::Unknown,
    is_large,
    m: 0,
    function: 0,
    capacity: 0,
    accept_data: false,
  })
}