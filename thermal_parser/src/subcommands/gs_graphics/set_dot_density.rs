use crate::{command::*, context::*};

#[derive(Clone)]
pub struct Handler;

impl CommandHandler for Handler {
    fn apply_context(&self, command: &Command, context: &mut Context) {
        let x = *command.data.get(0).unwrap_or(&50);
        let y = *command.data.get(1).unwrap_or(&50);

        if x == 50 && y == 50 { context.graphics.dots_per_inch = 180 } else if x == 51 && y == 51 { context.graphics.dots_per_inch = 360 }
    }
}

// Sets the reference dot density to process the graphics data or bit image data. (dpi: dots per inch)
pub fn new() -> Command {
    Command::new(
        "Set Dot Density",
        vec![1, 49],
        CommandType::Context,
        DataType::Subcommand,
        Box::new(Handler),
    )
}
