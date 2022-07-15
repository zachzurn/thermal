// pub mod constants;
pub mod commands;
pub mod command_sets;

use std::sync::Arc;

pub static ESC: u8 = 0x1B;
pub static HT: u8 = 0x09;
pub static LF: u8 = 0x0A;
pub static FF: u8 = 0x0C;
pub static CR: u8 = 0x0D;
pub static GS: u8 = 0x1D;
pub static FS: u8 = 0x1C;
pub static DLE: u8 = 0x10;
pub static CAN: u8 = 0x18;

#[derive(Clone)]
pub enum CommandType {
  Control,
  Text,
  TextContext,
  Image,
  Graphics,
  GraphicsContext,
  Unknown
}

#[derive(Clone)]
pub enum DataType {
  Empty,
  Single,
  Double,
  Triple,
  Text,
  Bitmap,
  Unknown
}

#[derive(Clone)]
struct ImageMeta {
  width: usize,
  height: usize,
  capacity: usize,
  size: usize,
  buffer: u8
}

#[derive(Clone)]
pub struct Command {
    pub commands: Vec<u8>,
    pub name: String,
    pub data: Vec<u8>,
    pub kind: CommandType,
    pub data_kind: DataType,
    pub handler: Arc<dyn CommandHandler>,
    image_meta: ImageMeta
}

pub trait CommandHandler {
    fn get_text(&self, _command: &Command) -> Option<String>{ None }
}

impl Command {
    pub fn new(name_str: &str, commands: Vec<u8>, kind: CommandType, data_kind: DataType, handler: Arc<dyn CommandHandler>) -> Self {
        let data: Vec<u8> = vec![];
        let name: String = name_str.to_string();
        let image_meta = ImageMeta { width: 0, height: 0, capacity: 0, size: 0, buffer: 0 };
        Self { commands, name, data, kind, data_kind, handler, image_meta }
    }

    // returns true if the byte was consumed or false if it was rejected
    pub fn push(&mut self, byte: u8) -> bool{
        let data_len =self.data.len();  

        match self.data_kind {
            DataType::Bitmap => {
              match self.data.len() {
                0 | 1 => {
                  self.data.push(byte); 
                  return true
                }
                2 => {
                  let p1 = self.data.pop().unwrap() as usize; //second byte
                  let m = self.data.pop().unwrap() as usize; //first byte
                  let p2 = byte as usize; //third byte

                  self.image_meta.width = p1 + p2 as usize * 256;
                  self.image_meta.height = 0;
                  self.image_meta.capacity = 0;
                  self.image_meta.size = 0;

                  if m == 32 || m == 33 {
                    self.image_meta.capacity = self.image_meta.width * 3;
                    self.image_meta.height = 24
                  } else {
                    self.image_meta.capacity = self.image_meta.width;
                    self.image_meta.height = 8;
                  }
                  return true
                }
                _ => {
                  //Here we are storing compressed image data since the bytes only ever contain 0 or 1
                  if self.image_meta.size >= self.image_meta.capacity { return false }
                  let bit_index = self.image_meta.size % 8;
                  
                                                       //set the nth bit to on
                  if byte > 0 { self.image_meta.buffer = (1 << bit_index) | self.image_meta.buffer }
                  
                  if bit_index == 7 { 
                    self.data.push(self.image_meta.buffer); 
                    self.image_meta.buffer = 0;
                  }
                  self.image_meta.size += 1;
                }
              }
            },
            DataType::Empty => return false,
            DataType::Single => if data_len >= 1 { return false },
            DataType::Double => if data_len >= 2 { return false },
            DataType::Triple => if data_len >= 3 { return false }
            DataType::Text | DataType::Unknown => {}, //Text and unknown collect bytes until the next match
        }
        self.data.push(byte); //Always push byte if not returned early
        true
    }

    pub fn arg(&self, at: usize) -> Option<u8>{
      match self.data.get(at) {
        Some(d) => { return Some(*d) }
        None => { return None }
      }
    }
}