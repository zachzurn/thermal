use crate::parser::context::Font;
use crate::parser::*;

#[derive(Clone)]
pub struct Handler;

impl CommandHandler for Handler {
    fn apply_context(&self, command: &Command, context: &mut Context) {
        let n = *command.data.get(0).unwrap_or(&0u8);
        context.code2d.composite_font = Font::from_raw(n);
    }
}

pub fn new() -> Command {
    Command::new(
        "Composite Sets Human Readable Options",
        vec![52, 72],
        CommandType::Subcommand,
        DataType::Subcommand,
        Box::new(Handler),
    )
}