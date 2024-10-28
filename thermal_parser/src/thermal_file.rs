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

use crate::constants::*;

pub static COMMENT_PREFIX: &str = "'//";
pub static HEX_PREFIX: &str = "0x";

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
                        let unescaped = token.replace("\\\"", "\"");
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
            //Allow for escaped quote
            if c == '\\' {
                esc_quoted = true;
                span.1 += 1;
                continue;
            }

            //
            if c == '\"' && esc_quoted {
                esc_quoted = false;
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
