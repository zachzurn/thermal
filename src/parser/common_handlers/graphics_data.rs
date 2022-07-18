use crate::parser::context::Context;
use crate::parser::{ CommandHandler, Command };
use crate::parser::graphics::*;

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
      self.capacity = u16::from_le_bytes([data[0], data[1]]) as u32;
      self.capacity -= 2;
      self.m = *data.get(2).unwrap();
      self.function = *data.get(3).unwrap();
      self.detect_kind();
    }

    if data_len == 6 {
      self.capacity = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
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
  fn get_graphics(&self, _command: &Command, _context: &Context) -> Option<GraphicsCommand> {
    match self.kind {
        GraphicsDataType::PrintGraphics => {
          return Some(GraphicsCommand::ImageRef(ImageRef{
            storage_id: self.m,
        }));
        },
        GraphicsDataType::StoreGraphics => {
          //calculate meta
          let _tone = _command.data.get(0).unwrap();
          let _color = _command.data.get(1).unwrap();
          let _width_mult = _command.data.get(2).unwrap();
          let _heigh_mult = _command.data.get(3).unwrap();
          let x1 = _command.data.get(4).unwrap();
          let x2 = _command.data.get(5).unwrap();
          let y1 = _command.data.get(6).unwrap();
          let y2 = _command.data.get(7).unwrap();

          let mut imagedata = _command.data.clone();
          imagedata.drain(0..8);

          return Some(GraphicsCommand::Image(Image{
            pixels: imagedata,
            width: *x1 as u32 + *x2 as u32 * 256,
            height: *y1 as u32 + *y2 as u32 * 256,
            pixel_type: PixelType::Byte,
            storage_id: Some(self.m),
          }))
        },
        GraphicsDataType::StoreColumnFmt => { return None },
        GraphicsDataType::Unknown => { return None },
    }
  }

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
  fn debug(&self, command: &Command, _context: &Context) -> String {
    let sub_description: &str;
    match self.kind {
      GraphicsDataType::PrintGraphics => sub_description = "Print Image",
      GraphicsDataType::StoreGraphics => sub_description = "Store Image",
      GraphicsDataType::StoreColumnFmt => sub_description = "Store Column Format",
      GraphicsDataType::Unknown => sub_description = "Unknown"
    }

    if matches!(self.kind, GraphicsDataType::Unknown){
      return format!("{} {} m({}) fun({}) capacity({}) {:02X?}", command.name, sub_description, self.m, self.function, self.capacity, command.data);
    }
    format!("{} {} with {} pixels", command.name, sub_description, self.capacity)
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