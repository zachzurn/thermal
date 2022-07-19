use crate::parser::*;

#[derive(Clone)]
pub struct Handler;

impl CommandHandler for Handler {}

// Sets the reference dot density to process the graphics data or bit image data. (dpi: dots per inch)
// [180 dpi × 180 dpi] is selected when x = 50 and y = 50
// [360 dpi × 360 dpi] is selected when x = 51 and y = 51
pub fn new() -> Command {
  Command::new(
    "Set Dot Density",
    vec![1, 49], 
    CommandType::Subcommand,
    DataType::Subcommand,
    Box::new(Handler)
  )
}