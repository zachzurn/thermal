use crate::parser::{*, graphics::ImageRefStorage};

#[derive(Clone)]
pub struct Handler;

impl CommandHandler for Handler {
    fn apply_context(&self, _command: &Command, context: &mut Context) {
        context.graphics.stored_graphics.retain(|k, _| {
            k.storage != ImageRefStorage::Disc
        });
    }
}

// Deletes all NV graphics data that has been defined using Functions 67 or 68.
// Deleted areas are designated "Unused areas".
// All key codes are designated as undefined.
pub fn new() -> Command {
    Command::new(
        "Clears NV Graphics Data",
        vec![65],
        CommandType::Subcommand,
        DataType::Subcommand,
        Box::new(Handler),
    )
}