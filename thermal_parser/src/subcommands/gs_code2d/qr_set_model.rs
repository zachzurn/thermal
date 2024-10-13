use crate::{command::*, context::*};

#[derive(Clone)]
pub struct Handler;

impl CommandHandler for Handler {
    fn apply_context(&self, command: &Command, context: &mut Context) {
        let n1 = *command.data.get(0).unwrap_or(&49u8);

        context.code2d.qr_model = match n1 {
            49 => QrModel::Model1,
            50 => QrModel::Model2,
            51 => QrModel::Micro,
            _ => QrModel::Model1,
        };
    }
}

pub fn new() -> Command {
    Command::new(
        "QR Sets the Model",
        vec![49, 65],
        CommandType::Context,
        DataType::Subcommand,
        Box::new(Handler),
    )
}
