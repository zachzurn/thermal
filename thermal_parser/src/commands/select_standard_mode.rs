use crate::context::Context;
use crate::{command::*, constants::*};

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {
    fn apply_context(&self, _command: &Command, context: &mut Context) {
        context.is_page_mode = false;
    }
}

pub fn new() -> Command {
    Command::new(
        "Select standard mode",
        vec![ESC, 'S' as u8],
        CommandType::Context,
        DataType::Empty,
        Box::new(Handler {}),
    )
}
