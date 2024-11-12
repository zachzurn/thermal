use crate::text::TextSpan;
use crate::{command::*, context::*};

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {
    fn get_text(&self, command: &Command, context: &Context) -> Option<TextSpan> {
        let decoded = context.text.decoder.decode_utf8(&command.data as &[u8]);
        Some(TextSpan::new(decoded, context))
    }
    fn debug(&self, command: &Command, context: &Context) -> String {
        format!(
            "{:?}",
            context.text.decoder.decode_utf8(&command.data as &[u8])
        )
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
