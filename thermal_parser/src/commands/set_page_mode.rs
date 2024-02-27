use crate::context::Context;
use crate::{command::*, constants::*};

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {
    fn apply_context(&self, _command: &Command, context: &mut Context) {
        context.is_page_mode = true;
    }
}

pub fn new() -> Command {
    Command::new(
        "Set page mode",
        vec![ESC, 'L' as u8],
        CommandType::Context,
        DataType::Empty,
        Box::new(Handler {}),
    )
}
