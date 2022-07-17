// pub mod constants;
pub mod commands;
pub mod command_sets;
pub mod common_handlers;

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
    pub handler: Box<dyn CommandHandler>
}

impl Command {
    pub fn new(name_str: &str, commands: Vec<u8>, kind: CommandType, data_kind: DataType, handler: Box<dyn CommandHandler>) -> Self {
        let data: Vec<u8> = vec![];
        let name: String = name_str.to_string();
        Self { commands, name, data, kind, data_kind, handler }
    }

    // returns true if the byte was consumed or false if it was rejected
    pub fn push(&mut self, byte: u8) -> bool{
        let data_len =self.data.len();  

        match self.data_kind {
            DataType::Custom => { 
              return self.handler.push(&mut self.data, byte)
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

//This seems insane, not quite sure how this actually works
//These next 3 traits/impl make the Box<dyn CommandHandler> cloneable
pub trait CloneCommandHandler {
  fn clone_command_handler<'a>(&self) -> Box<dyn CommandHandler>;
}

impl<T> CloneCommandHandler for T
where
    T: CommandHandler + Clone + 'static,
{
    fn clone_command_handler(&self) -> Box<dyn CommandHandler> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn CommandHandler> {
    fn clone(&self) -> Self {
        self.clone_command_handler()
    }
}

pub trait CommandHandler: CloneCommandHandler {
  fn get_text(&self, _command: &Command) -> Option<String>{ None }
  fn get_image_pbm(&self, _command: &Command) -> Option<Vec<u8>> { None }
  fn get_barcode(&self, _command: &Command) -> Option<AbstractBarcode> { None }
  
  fn debug(&self, _command: &Command) -> String { 
    if _command.data.is_empty() { return format!("{}", _command.name.to_string()) }
    format!("{} {:02X?}", _command.name.to_string(), _command.data) 
  }
  
  fn push(&mut self, _command: &mut Vec<u8>, _byte: u8) -> bool{ 
    return false 
  }
  //fn get image
  //fn get graphicscommands
}

pub struct AbstractBarcode {
  pub lines: Vec<u8>,
  pub text: String,
}