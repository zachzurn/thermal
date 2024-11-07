use crate::{command::*, constants::*, context::*, util::bitflags_lsb};

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {
    fn apply_context(&self, command: &Command, context: &mut Context) {
        let (font_b, _, _, bold, tall, wide, underline, _) =
            bitflags_lsb(command.data.get(0).unwrap_or(&0u8));

        context.set_font(if font_b { Font::B } else { Font::A });
        context.text.bold = bold;
        context.text.height_mult = if tall { 2 } else { 1 };
        context.text.width_mult = if wide { 2 } else { 1 };
        context.text.underline = if underline {
            TextUnderline::On
        } else {
            TextUnderline::Off
        };
    }

    fn debug(&self, command: &Command, _context: &Context) -> String {
        let mut changes = vec![];

        let (font_b, _, _, bold, tall, wide, underline, _) =
            bitflags_lsb(command.data.get(0).unwrap_or(&0u8));

        changes.push(if font_b { "Font B" } else { "Font A" });
        changes.push(if bold { "Bold" } else { "Not Bold" });
        changes.push(if tall {
            "Double Height"
        } else {
            "Standard Height"
        });
        changes.push(if wide {
            "Double Width"
        } else {
            "Standard Width"
        });
        changes.push(if underline {
            "Underline"
        } else {
            "No Underline"
        });

        format!("{}: {:?}", command.name, changes)
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
