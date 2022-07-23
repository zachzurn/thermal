use crate::{command::*, constants::*};
use crate::context::Context;
use crate::graphics::GraphicsCommand;

#[derive(Clone)]
struct Handler;

//See https://reference.epson-biz.com/modules/ref_escpos/index.php?content_id=34
impl CommandHandler for Handler {
    fn apply_context(&self, _command: &Command, context: &mut Context) {
        let n = *command.data.get(0).unwrap_or(&0u8);

        //last 4,5,6 bits masked
        context.text.height_mult = 0b00000111 & n + 1;

        //bit 1,2,3 masked and shifted all the way to the right
        context.text.width_mult = ((0b01110000 & value)>>4) + 1;
    }
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
