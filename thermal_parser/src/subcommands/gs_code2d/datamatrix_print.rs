use crate::{command::*, context::*, graphics::*};

#[derive(Clone)]
pub struct Handler;

impl CommandHandler for Handler {
    fn get_graphics(&self, _command: &Command, context: &Context) -> Option<GraphicsCommand> {
        match &context.code2d.symbol_storage {
            Some(code2d) => {
                return Some(GraphicsCommand::Code2D(code2d.clone()));
            }
            None => return None
        }
    }
}

pub fn new() -> Command {
    Command::new(
        "Print the Code2D data",
        vec![54, 81],
        CommandType::Subcommand,
        DataType::Subcommand,
        Box::new(Handler),
    )
}