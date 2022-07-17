mod parser;

use std::fs;

use crate::parser::{command_sets::esc_pos, CommandType};

fn main() {
    let bytes = std::fs::read("test/test_receipt.bin").unwrap();
    
    let esc_pos = esc_pos::new();
    let commands = esc_pos.parse(&bytes); 

    for command in commands {
        if matches!(command.kind, CommandType::Graphics) {
            if let Some(filedata) = command.handler.get_image_pbm(&command) {
                if let Ok(_) = fs::write("test/out.pbm", filedata) {
                };
            }
        }
        println!("{}", command.handler.debug(&command));
    }
}