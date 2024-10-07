use crate::graphics::TextSpan;
use crate::{command::*, constants::*, context::*};

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {
    fn get_text(&self, _command: &Command, context: &Context) -> Option<TextSpan> {
        Some(TextSpan::new("\r".to_string(), context))
    }
}

pub fn new() -> Command {
    Command::new(
        "Line Feed",
        vec![CR],
        CommandType::Text,
        DataType::Empty,
        Box::new(Handler {}),
    )
}
