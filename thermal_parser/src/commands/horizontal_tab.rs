use crate::{command::*, constants::*, context::*};

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {
    fn get_text(&self, _command: &Command, _context: &Context) -> Option<String> {
        Some("\t".to_string())
    }
}

pub fn new() -> Command {
    Command::new(
        "Horizontal Tab",
        vec![HT],
        CommandType::Text,
        DataType::Empty,
        Box::new(Handler {}),
    )
}
