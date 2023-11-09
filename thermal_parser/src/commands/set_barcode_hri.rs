use crate::{command::*, constants::*, context::*};

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {
    fn apply_context(&self, command: &Command, context: &mut Context) {
        let n = *command.data.get(0).unwrap_or(&0u8);
        context.barcode.human_readable = match n {
            1 | 49 => HumanReadableInterface::Above,
            2 | 50 => HumanReadableInterface::Below,
            3 | 51 => HumanReadableInterface::Both,
            _ => HumanReadableInterface::None,
        }
    }
}

//Position of Human Readable characters
pub fn new() -> Command {
    Command::new(
        "Set Hri Print POS",
        vec![GS, 'H' as u8],
        CommandType::Context,
        DataType::Single,
        Box::new(Handler {}),
    )
}
