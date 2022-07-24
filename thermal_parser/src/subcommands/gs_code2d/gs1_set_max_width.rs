use crate::{command::*, context::*};

#[derive(Clone)]
pub struct Handler;

impl CommandHandler for Handler {
    fn apply_context(&self, command: &Command, context: &mut Context) {
        let nl = *command.data.get(0).unwrap_or(&0u8);
        let nh = *command.data.get(1).unwrap_or(&0u8);
        context.code2d.gs1_databar_max_width = nl as u32 + nh as u32 * 256;
    }
}

pub fn new() -> Command {
    Command::new(
        "GS1 Sets Max Width",
        vec![51, 71],
        CommandType::Context,
        DataType::Subcommand,
        Box::new(Handler),
    )
}
