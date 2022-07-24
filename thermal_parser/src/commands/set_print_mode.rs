use crate::{command::*, constants::*, context::*, util::bitflags_msb};

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {
    fn apply_context(&self, command: &Command, context: &mut Context) {
        let n = *command.data.get(0).unwrap_or(&1u8);
        let bits = bitflags_msb(n);

        context.text.font = if bits[0] { Font::B } else { Font::A };
        context.text.bold = if bits[3] { true } else { false };
        context.text.height_mult = if bits[4] { 2 } else { 1 };
        context.text.width_mult = if bits[5] { 2 } else { 1 };
        context.text.underline = if bits[7] { TextUnderline::On } else { TextUnderline::Off }
    }
}

pub fn new() -> Command {
    Command::new(
        "Set Print Mode",
        vec![ESC, '!' as u8],
        CommandType::Context,
        DataType::Single,
        Box::new(Handler {}),
    )
}
