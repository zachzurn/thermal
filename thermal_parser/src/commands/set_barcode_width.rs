//! Set Barcode Width
//! GS W
//!
//! Sets the barcode line and space between width. Minimum is 2 and max is 6.
//! Values that fall outside of this range will be brought into range.
//!
use crate::{command::*, constants::*, context::*};

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {
    fn apply_context(&self, command: &Command, context: &mut Context) {
        let mut n = *command.data.get(0).unwrap_or(&2u8);
        if n < 2 {
            n = 2
        };
        
        // Not sure what the expected behavior is when 
        // the width is set to higher than the max of 6. 
        // 
        // For now we ignore and leave the default, 
        // which seems to produce expected output.
        if n > 6 {
            return
        };
        
        context.barcode.width = n;
    }
}

pub fn new() -> Command {
    Command::new(
        "Set Barcode Width",
        vec![GS, 'w' as u8],
        CommandType::Context,
        DataType::Single,
        Box::new(Handler {}),
    )
}
