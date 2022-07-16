// pub mod constants;
pub mod commands;
pub mod command_sets;
pub mod common_handlers;

use std::sync::{Arc, Mutex};

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
  Custom, //Command Handler implements the push logic
  Unknown
}

#[derive(Clone)]
pub struct Command {
    pub commands: Vec<u8>,
    pub name: String,
    pub data: Vec<u8>,
    pub kind: CommandType,
    pub data_kind: DataType,
    pub handler: Arc<Mutex<dyn CommandHandler>>
}

pub trait CommandHandler {
    fn get_text(&self, _command: &Command) -> Option<String>{ None }
    fn push(&mut self, _command: &mut Vec<u8>, _byte: u8) -> bool{ return false }
    //fn get image
    //fn get graphicscommands
}

impl Command {
    pub fn new(name_str: &str, commands: Vec<u8>, kind: CommandType, data_kind: DataType, handler: Arc<Mutex<dyn CommandHandler>>) -> Self {
        let data: Vec<u8> = vec![];
        let name: String = name_str.to_string();
        Self { commands, name, data, kind, data_kind, handler }
    }

    // returns true if the byte was consumed or false if it was rejected
    pub fn push(&mut self, byte: u8) -> bool{
        let data_len =self.data.len();  

        match self.data_kind {
            DataType::Custom => { 
              return self.handler.lock().unwrap().push(&mut self.data, byte)
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
}