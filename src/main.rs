mod parser;

use crate::parser::CommandType;
use crate::parser::command_sets::esc_pos;

fn main() {
    let bytes = std::fs::read("test/receipt-with-logo.bin").unwrap();
    
    let esc_pos = esc_pos::new();
    let commands = esc_pos.parse(&bytes); 

    // let commands = parse_commands(&bytes, esc_pos);

    for command in commands {
        match command.kind {
            CommandType::Text => {
                if let Some(str) = command.handler.lock().unwrap().get_text(&command) { print!("{}", str) }
            }
            _default => {
                println!("-- {} {:02X?}", command.name, command.data);
            }
        }
    }

    //print!("{:?}", commands);
}