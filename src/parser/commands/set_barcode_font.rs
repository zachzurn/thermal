use crate::parser::*;
use crate::parser::context::Font;

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {
    fn apply_context(&self, command: &Command, context: &mut Context) {
        let n = *command.data.get(0).unwrap_or(&0u8);
        context.barcode.font = Font::from_raw(n);
    }
}

//Position of Human Readable characters
pub fn new() -> Command {
    Command::new(
        "Set Hri Barcode Font",
        vec![GS, 'f' as u8],
        CommandType::Context,
        DataType::Single,
        Box::new(Handler {}),
    )
}