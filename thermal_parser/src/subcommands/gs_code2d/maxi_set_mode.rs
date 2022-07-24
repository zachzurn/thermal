use crate::{command::*, context::*};

#[derive(Clone)]
pub struct Handler;

impl CommandHandler for Handler {
    fn apply_context(&self, command: &Command, context: &mut Context) {
        let n = *command.data.get(0).unwrap_or(&50u8);

        //mode 2 through 6
        context.code2d.maxicode_mode = n - 48;
    }
}

pub fn new() -> Command {
    Command::new(
        "Maxi Sets the Mode",
        vec![50, 65],
        CommandType::Context,
        DataType::Subcommand,
        Box::new(Handler),
    )
}
