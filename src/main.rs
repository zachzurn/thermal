mod parser;

use crate::parser::CommandType;
use crate::parser::command_sets::esc_pos;

fn main() {
    let bytes = std::fs::read("test/receipt-with-logo.bin").unwrap();
    
    let esc_pos = esc_pos::get();
    let commands = esc_pos.parse_commands(&bytes); 

    // let commands = parse_commands(&bytes, esc_pos);

    for command in commands {
        match command.kind {
            CommandType::Text => {
                if let Some(str) = command.handler.get_text(&command) { println!("{}", str) }
            }
            CommandType::Unknown | CommandType::TextContext => {
                println!("{} {:02X?}", command.name, command.data);
            }
            _default => {
                println!("{}",command.name);
            }
        }
    }

    //print!("{:?}", commands);
}