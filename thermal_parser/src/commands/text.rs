use crate::{command::*, context::*};

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {
    fn get_text(&self, command: &Command, context: &Context) -> Option<String> {
        let decoded = context.text.decoder.decode_utf8(&command.data as &[u8]);
        Some(decoded)
    }
    fn debug(&self, command: &Command, context: &Context) -> String {
        self.get_text(command, context).unwrap_or("".to_string())
    }
}

pub fn new() -> Command {
    Command::new(
        "Text",
        vec![],
        CommandType::Text,
        DataType::Text,
        Box::new(Handler {}),
    )
}
