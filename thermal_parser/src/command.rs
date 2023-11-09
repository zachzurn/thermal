use crate::context::Context;
use crate::graphics::GraphicsCommand;
use std::rc::Rc;

#[derive(Clone, PartialEq)]
pub enum DeviceCommand {
    BeginPrint,
    Initialize,
    PartialCut,
    FullCut,
    Feed(i16),
    FeedLine(i16),
    Cancel,
    Pulse,
    EndPrint,
    Transmit(Vec<u8>),
    MoveX(u16),
}

impl DeviceCommand {
    pub fn as_string(&self) -> String {
        match self {
            Self::Initialize => "Initialize".to_string(),
            Self::PartialCut => "Partial Cut".to_string(),
            Self::FullCut => "Full Cut".to_string(),
            Self::Feed(n) => format!("Feed {} Motion Units", n),
            Self::FeedLine(n) => format!("Feed {} Lines", n),
            Self::Cancel => "Cancel".to_string(),
            Self::Pulse => "Pulse".to_string(),
            Self::EndPrint => "End Print".to_string(),
            Self::BeginPrint => "Begin Print".to_string(),
            Self::Transmit(_b) => "Transmit Data Back".to_string(),
            Self::MoveX(_n) => "Move Horizontally".to_string(),
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum CommandType {
    Control,
    Text,
    Graphics,
    Context,
    ContextControl,
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
    pub commands: Rc<Vec<u8>>,
    pub name: Rc<String>,
    pub data: Vec<u8>,
    pub kind: CommandType,
    pub data_kind: DataType,
    pub handler: Box<dyn CommandHandler>,
}

#[derive(Clone)]
struct EmptyHandler;
impl CommandHandler for EmptyHandler {}

impl Command {
    pub fn new(
        name_str: &str,
        commands: Vec<u8>,
        kind: CommandType,
        data_kind: DataType,
        handler: Box<dyn CommandHandler>,
    ) -> Self {
        let data: Vec<u8> = vec![];
        let name: String = name_str.to_string();
        Self {
            commands: Rc::new(commands),
            name: Rc::new(name),
            data,
            kind,
            data_kind,
            handler,
        }
    }

    // returns true if the byte was consumed or false if it was rejected
    pub fn push(&mut self, byte: u8) -> bool {
        let data_len = self.data.len();

        match self.data_kind {
            DataType::Custom | DataType::Subcommand => {
                return self.handler.push(&mut self.data, byte);
            }
            DataType::Empty => return false,
            DataType::Single => {
                if data_len >= 1 {
                    return false;
                }
            }
            DataType::Double => {
                if data_len >= 2 {
                    return false;
                }
            }
            DataType::Triple => {
                if data_len >= 3 {
                    return false;
                }
            }
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
    fn get_text(&self, _command: &Command, _context: &Context) -> Option<String> {
        None
    }

    //Renders a graphic
    fn get_graphics(&self, _command: &Command, _context: &Context) -> Option<GraphicsCommand> {
        None
    }

    //Applies context
    fn apply_context(&self, _command: &Command, _context: &mut Context) {}

    //Gets a device command to execute
    fn get_device_command(
        &self,
        _command: &Command,
        _context: &Context,
    ) -> Option<Vec<DeviceCommand>> {
        None
    }

    //For debugging commands
    fn debug(&self, command: &Command, _context: &Context) -> String {
        if command.data.is_empty() {
            return format!("{}", command.name.to_string());
        }
        format!("{} {:02X?}", command.name.to_string(), command.data)
    }

    //Push data to a command. The command decides what to accept
    fn push(&mut self, _command: &mut Vec<u8>, _byte: u8) -> bool {
        return false;
    }

    //Returns the subcommand for a command, see subcommand module
    fn get_subcommand(&mut self) -> Option<Command> {
        None
    }
}
