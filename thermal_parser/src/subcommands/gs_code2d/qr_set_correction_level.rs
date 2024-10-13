use crate::{command::*, context::*};

#[derive(Clone)]
pub struct Handler;

impl CommandHandler for Handler {
    fn apply_context(&self, command: &Command, context: &mut Context) {
        let n = *command.data.get(0).unwrap_or(&48u8);

        context.code2d.qr_error_correction = match n {
            1 => QrErrorCorrection::M,
            2 => QrErrorCorrection::Q,
            3 => QrErrorCorrection::H,
            _ => QrErrorCorrection::L,
        }
    }
}

pub fn new() -> Command {
    Command::new(
        "QR Sets Error Correction Level",
        vec![49, 69],
        CommandType::Context,
        DataType::Subcommand,
        Box::new(Handler),
    )
}
