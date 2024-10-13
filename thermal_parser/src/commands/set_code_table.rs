//! Sets the code table to use for displaying text
//!
//! These are the code tables that can be set.
//! Support for these is limited to the code tables that
//! we have gathered in the decoder mod.
//!
//! See: https://download4.epson.biz/sec_pubs/pos/reference_en/charcode/index.html
//!
//! Page 0 [PC437: USA, Standard Europe]
//! Page 1 [Katakana]
//! Page 2 [PC850: Multilingual]
//! Page 3 [PC860: Portuguese]
//! Page 4 [PC863: Canadian-French]
//! Page 5 [PC865: Nordic]
//! Page 6 [Hiragana]
//! Page 7 [One-pass printing Kanji characters]
//! Page 8 [One-pass printing Kanji characters]
//! Page 11 [PC851: Greek]
//! Page 12 [PC853: Turkish]
//! Page 13 [PC857: Turkish]
//! Page 14 [PC737: Greek]
//! Page 15 [ISO8859-7: Greek]
//! Page 16 [WPC1252]
//! Page 17 [PC866: Cyrillic #2]
//! Page 18 [PC852: Latin 2]
//! Page 19 [PC858: Euro]
//! Page 20 [Thai Character Code 42]
//! Page 21 [Thai Character Code 11]
//! Page 22 [Thai Character Code 13]
//! Page 23 [Thai Character Code 14]
//! Page 24 [Thai Character Code 16]
//! Page 25 [Thai Character Code 17]
//! Page 26 [Thai Character Code 18]
//! Page 30 [TCVN-3: Vietnamese]
//! Page 31 [TCVN-3: Vietnamese]
//! Page 32 [PC720: Arabic]
//! Page 33 [WPC775: Baltic Rim]
//! Page 34 [PC855: Cyrillic]
//! Page 35 [PC861: Icelandic]
//! Page 36 [PC862: Hebrew]
//! Page 37 [PC864: Arabic]
//! Page 38 [PC869: Greek]
//! Page 39 [ISO8859-2: Latin 2]
//! Page 40 [ISO8859-15: Latin 9]
//! Page 41 [PC1098: Farsi]
//! Page 42 [PC1118: Lithuanian]
//! Page 43 [PC1119: Lithuanian]
//! Page 44 [PC1125: Ukrainian]
//! Page 45 [WPC1250: Latin 2]
//! Page 46 [WPC1251: Cyrillic]
//! Page 47 [WPC1253: Greek]
//! Page 48 [WPC1254: Turkish]
//! Page 49 [WPC1255: Hebrew]
//! Page 50 [WPC1256: Arabic]
//! Page 51 [WPC1257: Baltic Rim]
//! Page 52 [WPC1258: Vietnamese]
//! Page 53 [KZ-1048: Kazakhstan]
//! Page 66 [Devanagari]
//! Page 67 [Bengali]
//! Page 68 [Tamil]
//! Page 69 [Telugu]
//! Page 70 [Assamese]
//! Page 71 [Oriya]
//! Page 72 [Kannada]
//! Page 73 [Malayalam]
//! Page 74 [Gujarati]
//! Page 75 [Punjabi]
//! Page 82 [Marathi]
//! Page 254
//! Page 255 We are using this for unicode

use crate::{command::*, constants::*, context::*};

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {
    fn apply_context(&self, command: &Command, context: &mut Context) {
        let n = *command.data.get(0).unwrap_or(&0u8);
        context.text.code_table = n;
        context.update_decoder();
    }

    fn debug(&self, command: &Command, context: &Context) -> String {
        format!(
            "{} to {}. {} ({})",
            command.name,
            context.text.code_table,
            context.text.decoder.name,
            context.text.decoder.language
        )
    }
}

pub fn new() -> Command {
    Command::new(
        "Set Code Table",
        vec![ESC, 't' as u8],
        CommandType::Context,
        DataType::Single,
        Box::new(Handler {}),
    )
}
