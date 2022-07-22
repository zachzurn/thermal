use crate::context::Context;
use crate::graphics::GraphicsCommand;

#[derive(Clone, PartialEq)]
pub enum CommandType {
    Initialize,
    Control,
    Text,
    Graphics,
    Context,
    Subcommand,
    Unknown,
}

#[derive(Clone, PartialEq)]
pub enum DataType {
    Empty,
    Single,
    Double,
    Triple,
    Text,
    Custom,
    Subcommand,
    Unknown,
}

#[derive(Clone)]
pub struct Command {
    pub commands: Vec<u8>,
    pub name: String,
    pub data: Vec<u8>,
    pub kind: CommandType,
    pub data_kind: DataType,
    pub handler: Box<dyn CommandHandler>,
}

impl Command {
    pub fn new(name_str: &str, commands: Vec<u8>, kind: CommandType, data_kind: DataType, handler: Box<dyn CommandHandler>) -> Self {
        let data: Vec<u8> = vec![];
        let name: String = name_str.to_string();
        Self { commands, name, data, kind, data_kind, handler }
    }

    // returns true if the byte was consumed or false if it was rejected
    pub fn push(&mut self, byte: u8) -> bool {
        let data_len = self.data.len();

        match self.data_kind {
            DataType::Custom | DataType::Subcommand => {
                return self.handler.push(&mut self.data, byte);
            }
            DataType::Empty => return false,
            DataType::Single => if data_len >= 1 { return false; },
            DataType::Double => if data_len >= 2 { return false; },
            DataType::Triple => if data_len >= 3 { return false; }
            DataType::Text | DataType::Unknown => {} //Text and unknown collect bytes until the next match
        }
        self.data.push(byte); //Always push byte if not returned early
        true
    }
}

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
    //Renders text
    fn get_text(&self, _command: &Command, _context: &Context) -> Option<String> { None }

    //Renders a graphic
    fn get_graphics(&self, _command: &Command, _context: &Context) -> Option<GraphicsCommand> { None }

    //Applies context
    fn apply_context(&self, _command: &Command, _context: &mut Context) {}

    //Transmits data back to the client
    fn transmit(&self, _command: &Command, _context: &Context) -> Option<Vec<u8>> { None }

    //For debugging commands
    fn debug(&self, command: &Command, _context: &Context) -> String {
        if command.data.is_empty() { return format!("{}", command.name.to_string()); }
        format!("{} {:02X?}", command.name.to_string(), command.data)
    }

    //Push data to a command. The command decides what to accept
    fn push(&mut self, _command: &mut Vec<u8>, _byte: u8) -> bool {
        return false;
    }
}