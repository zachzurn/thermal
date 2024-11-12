//! Sets the international character set.
//!
//! The base Codepage will be different based on the international character set.
//!
//! An example of how this works. In the USA character set a # sign would be the £ character instead.
//!
//! See: https://download4.epson.biz/sec_pubs/pos/reference_en/charcode/international.html
//!
//! 0 U.S.A.
//! 1 France
//! 2 Germany
//! 3 U.K.
//! 4 Denmark I
//! 5 Sweden
//! 6 Italy
//! 7 Spain I
//! 8 Japan
//! 9 Norway
//! 10 Denmark II
//! 11 Spain II
//! 12 Latin America
//! 13 Korea
//! 14 Slovenia / Croatia
//! 15 China
//! 16 Vietnam
//! 17 Arabia
//! 66 India (Devanagari)
//! 67 India (Bengali)
//! 68 India (Tamil)
//! 69 India (Telugu)
//! 70 India (Assamese)
//! 71 India (Oriya)
//! 72 India (Kannada)
//! 73 India (Malayalam)
//! 74 India (Gujarati)
//! 75 India (Punjabi)
//! 82 India (Marathi)

use crate::{command::*, constants::*, context::*};

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {
    fn apply_context(&self, command: &Command, context: &mut Context) {
        let n = *command.data.get(0).unwrap_or(&0u8);
        context.text.character_set = n;
        context.update_decoder();
    }

    fn debug(&self, command: &Command, context: &Context) -> String {
        format!(
            "{} to {}. {} ({})",
            command.name,
            context.text.character_set,
            context.text.decoder.name,
            context.text.decoder.language
        )
    }
}

pub fn new() -> Command {
    Command::new(
        "Set International Character Set",
        vec![ESC, 'R' as u8],
        CommandType::TextStyle,
        DataType::Single,
        Box::new(Handler {}),
    )
}
