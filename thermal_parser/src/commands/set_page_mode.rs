use crate::{command::*, constants::*};
use crate::context::{Context};

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {
    fn apply_context(&self, command: &Command, context: &mut Context) {
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