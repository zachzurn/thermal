use crate::{command::*, constants::*, context::*, util::bitflags_lsb};

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {
    fn apply_context(&self, command: &Command, context: &mut Context) {
        let n = *command.data.get(0).unwrap_or(&1u8);

        //no bits, reset everything
        if n == 0 {
            context.text.font = Font::A;
            context.text.bold = false;
            context.text.height_mult = 1;
            context.text.width_mult = 1;
            context.text.underline = TextUnderline::Off;
        }

        //Bits only enable
        let bits = bitflags_lsb(n);

        context.text.font = if bits[0] { Font::B } else { Font::A };

        if bits[3] {
            context.text.bold = true;
        }
        if bits[4] {
            context.text.height_mult = 2;
        }
        if bits[5] {
            context.text.width_mult = 2;
        }
        if bits[7] {
            context.text.underline = TextUnderline::On;
        }
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
