//! Parser and converter of .thermal file format.
//!
//! This file format is commonly used in programming
//! examples and is easy for humans to read.
//!
//! This is a great way to make test files.
//!
//!
//! Thermal file format:
//!
//! ```'// Comments look like this
//! "Quoted values are strings"
//! "Hex values look like this ->" 0xFF
//! "Decimal Values look like this -> " 23
//! "There are a few constant values that can also be used"
//! NUL ESC HT LF FF CR GS FS DLE CAN "Are all valid"```
//!
//!
//! Some examples:
//!
//! ```
//! '// Initialize
//! ESC "@"
//!
//! '// Align center
//! ESC "a" 1
//!
//! '// Print some text
//! This should be centered
//!
//! '// Print barcode: (A) format, barcode system = CODE39
//! GS "k" 4 "*00014*" 0
//!
//! '// Select cut mode and cut paper: [Function B] Feed paper to
//! GS "V" 66 30
//!
//! ```

use crate::command::{Command, CommandType};
use crate::constants;
use crate::constants::*;
use std::fmt::format;
use std::rc::Rc;

pub static COMMENT_PREFIX: &str = "'//";
pub static HEX_PREFIX: &str = "0x";

/// Parse thermal format from string into bytes
pub fn parse_str(text: &str) -> Vec<u8> {
    let mut parsed = Vec::new();

    for line in text.lines() {
        //skip comments
        if line.starts_with(COMMENT_PREFIX) || line.trim().is_empty() {
            continue;
        }

        //Parse tokens
        let tokens = parse_tokens(line);

        //Convert tokens to bytes
        for token in tokens {
            match token {
                "NUL" => parsed.push(NUL),
                "ESC" => parsed.push(ESC),
                "HT" => parsed.push(HT),
                "LF" => parsed.push(LF),
                "FF" => parsed.push(FF),
                "CR" => parsed.push(CR),
                "GS" => parsed.push(GS),
                "FS" => parsed.push(FS),
                "DLE" => parsed.push(DLE),
                "CAN" => parsed.push(CAN),
                _ => {
                    //Hex 0xFF for example
                    if token.starts_with(HEX_PREFIX) {
                        let maybe_byte = u8::from_str_radix(&token[2..], 16);
                        if let Ok(byte) = maybe_byte {
                            parsed.push(byte);
                        }
                    }
                    //raw strings start with quote
                    else if token.starts_with('"') {
                        let unescaped = token.replace("\\\\", "\\").replace("\\\"", "\"");
                        for byte in unescaped[1..].as_bytes() {
                            parsed.push(*byte);
                        }
                    }
                    //Decimal
                    else {
                        let maybe_decimal = token.parse::<u8>();

                        //Can parse decimal from string
                        if let Ok(decimal) = maybe_decimal {
                            parsed.push(decimal)
                        }
                        //Cannot parse decimal, output raw
                        else {
                            for byte in token.as_bytes() {
                                parsed.push(*byte);
                            }
                        }
                    }
                }
            }
        }
    }

    parsed
}

pub fn parse_tokens(line: &str) -> Vec<&str> {
    let mut tokens = Vec::new();
    let mut span = (0, 0);
    let mut gobble_quoted = false;
    let mut esc_quoted = false;

    for c in line.chars() {
        //Awaiting quoted string
        if gobble_quoted {
            //Allow for escaping backslashes
            if c == '\\' && esc_quoted {
                esc_quoted = false;
                span.1 += 1;
                continue;
            }

            //
            if c == '\"' && esc_quoted {
                esc_quoted = false;
                span.1 += 1;
                continue;
            }

            //Allow for escaped quote
            if c == '\\' {
                esc_quoted = true;
                span.1 += 1;
                continue;
            }

            esc_quoted = false;

            //End quote, push the string
            if c == '"' {
                tokens.push(&line[span.0..span.1]);
                span.1 += c.len_utf8();
                span.0 = span.1;
                gobble_quoted = false;
            }
            //Still gobbling
            else {
                span.1 += c.len_utf8()
            }

            continue;
        }

        //Start gobbling quoted string
        if c == '"' {
            gobble_quoted = true;
            esc_quoted = false;

            //Here we include the first quote
            span.0 = span.1;
            span.1 += 1;

            continue;
        }

        //Awaiting whitespace to end token
        if c.is_ascii_whitespace() {
            //See if there is a token to push
            if span.0 != span.1 {
                tokens.push(&line[span.0..span.1])
            }

            //Move the span
            span.1 += c.len_utf8();
            span.0 = span.1;
        } else {
            //Move only the end of the span
            span.1 += c.len_utf8();
        }
    }

    //Check if there is an eligible span left
    if span.0 != span.1 {
        tokens.push(&line[span.0..span.1])
    }

    tokens
}

pub fn parse_binary(_bytes: Vec<u8>) -> Vec<String> {
    let lines: Vec<String> = vec![];
    lines
}

/// Utility for converting commands into the human readable
/// Thermal file format.
pub fn cmds_to_thermal(cmds: &Vec<Command>) -> String {
    let mut thermal = vec![];

    for cmd in cmds {
        thermal.push(cmd_to_thermal(cmd));
    }

    thermal.join("")
}

fn cmd_to_thermal(cmd: &Command) -> String {
    if cmd.kind == CommandType::Text {
        if cmd.commands.len() > 0 && cmd.commands[0] == constants::LF {
            return "LF \n".to_string();
        }

        let text = String::from_utf8_lossy(cmd.data.as_slice());
        return format!("\"{}\"\n", text);
    } else if cmd.kind == CommandType::Unknown {
        return format!("'// Unknown command \n {}\n\n", explain_unknown(&cmd)).to_string();
    }

    let mut lines: Vec<String> = vec![];

    //Add comment explaining command
    lines.push(format!("'// {}", cmd.name).to_string());

    //Convert command bytes to constant and decimal
    let mut cmd_str = explain_command(&cmd.commands);

    //Small data lists are best expressed as digits
    if cmd.data.len() < 10 {
        for b in cmd.data.clone().into_iter() {
            cmd_str.push_str(&format!(" {}", b));
        }
    //Large data lists are often graphics and best expressed with hex
    } else {
        cmd_str.push_str("\n");
        for chunk in cmd.data.chunks(32) {
            let mut data_str = String::new();
            for b in chunk {
                //add bytes as 0xFF with a space after
                data_str.push_str(&format!(" 0x{:02X}", b));
            }

            cmd_str.push_str(&data_str.trim());
            cmd_str.push_str("\n");
        }
    }

    lines.push(cmd_str);

    format!("{}\n\n", lines.join("\n"))
}

pub fn try_const(byte: &u8) -> String {
    match byte {
        0x1B => "ESC".to_string(),
        0x1D => "GS".to_string(),
        0x1C => "FS".to_string(),
        0x0C => "FF".to_string(),
        0x00 => "NUL".to_string(),
        0x0D => "CR".to_string(),
        0x10 => "DLE".to_string(),
        0x18 => "CAN".to_string(),
        _ => format!("0x{:02X}", byte),
    }
}

static BACKSLASH: &u8 = &0x5C;
static QUOTE: &u8 = &0x22;

pub fn try_string(byte: &u8) -> String {
    if byte.is_ascii_control() {
        format!("{}", byte)
    } else if byte == BACKSLASH {
        //Escape the backslash
        // = "\\"
        "\"\\\\\"".to_string()
    } else if byte == QUOTE {
        // Escape the quote
        // = "\""
        "\"\\\"\"".to_string()
    } else {
        let str = vec![*byte];
        let char = String::from_utf8_lossy(str.as_slice());
        format!("\"{}\"", char)
    }
}

fn explain_unknown(cmd: &Command) -> String {
    if cmd.commands.len() > 1 {
        let first = &cmd.data[0];
        let second = &cmd.data[1];
        let rest = &cmd.data[2..];

        //Normally the first byte is a const value
        let mut str = try_const(first);

        str.push_str(" ");

        //Normally the second byte is best expressed as a string
        str.push_str(&try_string(second));

        str.push_str(" ");

        for b in rest {
            str.push_str(&try_const(b));
            str.push_str(" ");
        }

        return str;
    } else if cmd.data.len() == 1 {
        return try_const(&cmd.data[0]);
    }
    "'// No bytes provided".to_string()
}

fn explain_command(rc: &Rc<Vec<u8>>) -> String {
    if rc.is_empty() {
        return "".to_string();
    } else {
        let first = rc[0];
        let rest = &rc[1..];

        let mut first_str: String = try_const(&first);
        first_str.push_str(" ");

        for byte in rest {
            first_str.push_str(&try_string(byte));
            first_str.push_str(" ");
        }

        first_str
    }
}
