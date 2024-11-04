use crate::{command::*, constants::*, context::*};

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {
    fn apply_context(&self, _command: &Command, context: &mut Context) {
        //context.reset_x();

        if context.page_mode.enabled {
            //context.reset_y();
        }
    }
}

pub fn new() -> Command {
    Command::new(
        "Unknown GS ( G",
        vec![GS, '(' as u8, 'G' as u8],
        CommandType::Context,
        DataType::Quad,
        Box::new(Handler {}),
    )
}
