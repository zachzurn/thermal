use crate::{command::*, constants::*, context::*};

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {
    fn get_text(&self, _command: &Command, _context: &Context) -> Option<String> {
        Some("\x0c".to_string())
    }
}

pub fn new() -> Command {
    Command::new(
        "Form Feed",
        vec![FF],
        CommandType::Text,
        DataType::Empty,
        Box::new(Handler {}),
    )
}
