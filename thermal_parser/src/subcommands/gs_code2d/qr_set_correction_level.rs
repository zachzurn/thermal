use crate::{command::*, context::*};

#[derive(Clone)]
pub struct Handler;

impl CommandHandler for Handler {
    fn apply_context(&self, command: &Command, context: &mut Context) {
        let n = *command.data.get(0).unwrap_or(&48u8);

        //0 = L
        //1 = M
        //2 = Q
        //3 = H
        context.code2d.qr_model = n - 48;
    }
}

pub fn new() -> Command {
    Command::new(
        "QR Sets Error Correction Level",
        vec![49, 69],
        CommandType::Subcommand,
        DataType::Subcommand,
        Box::new(Handler),
    )
}