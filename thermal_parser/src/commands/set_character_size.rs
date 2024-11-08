use crate::{command::*, constants::*, context::*};

#[derive(Clone)]
struct Handler;

//See https://reference.epson-biz.com/modules/ref_escpos/index.php?content_id=34
impl CommandHandler for Handler {
    fn apply_context(&self, command: &Command, context: &mut Context) {
        let n = *command.data.get(0).unwrap_or(&0u8);
        let stretch = parse_stretch(n);
        context.text.width_mult = stretch.0;
        context.text.height_mult = stretch.1;
    }

    fn debug(&self, command: &Command, _context: &Context) -> String {
        let n = *command.data.get(0).unwrap_or(&0u8);
        let stretch = parse_stretch(n);
        format!("{} Stretch w{} h{}", command.name, stretch.0, stretch.1)
    }
}

fn parse_stretch(value: u8) -> (u8, u8) {
    //last 4,5,6 bits masked
    //   ---
    //11110101 -> 00000101
    let h = 0b00000111 & value + 1;

    //bit 1,2,3 masked and shifted all the way to the right // ***
    // ---
    //01010101 -> 01010000 -> 00000101
    let w = ((0b01110000 & value) >> 4) + 1;

    (w,h)
}

pub fn new() -> Command {
    Command::new(
        "Set Character Size",
        vec![GS, '!' as u8],
        CommandType::Context,
        DataType::Single,
        Box::new(Handler {}),
    )
}
