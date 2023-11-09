use crate::context::{Context, TextStrikethrough};
use crate::{command::*, constants::*};

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {
    fn apply_context(&self, command: &Command, context: &mut Context) {
        let n = *command.data.get(0).unwrap_or(&0u8);
        let enable = (n & 0x00000001) == 1;
        let mut strike = context.text.strikethrough.clone();

        if enable {
            strike = TextStrikethrough::Double
        } else if strike == TextStrikethrough::Double {
            strike = TextStrikethrough::On
        }

        context.text.strikethrough = strike;
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
