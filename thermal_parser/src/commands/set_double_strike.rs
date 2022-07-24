use crate::{command::*, constants::*};
use crate::context::{Context, TextUnderline};

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {
    fn apply_context(&self, command: &Command, context: &mut Context) {
        let n = *command.data.get(0).unwrap_or(&0u8);
        let enable = (n & 0x00000001) == 1;
        let mut underline = context.text.underline.clone();

        if enable { underline = TextUnderline::Double }
        else if underline == TextUnderline::Double { underline = TextUnderline::On }

        context.text.underline = underline;
    }
}

pub fn new() -> Command {
    Command::new(
        "Enable Double Strike Through",
        vec![ESC, 'G' as u8],
        CommandType::Context,
        DataType::Single,
        Box::new(Handler {}),
    )
}
