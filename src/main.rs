mod parser;

use crate::parser::command_sets::esc_pos;

fn main() {
    let bytes = std::fs::read("test/receipt-with-logo.bin").unwrap();
    
    let esc_pos = esc_pos::new();
    let commands = esc_pos.parse(&bytes); 

    for command in commands {
        println!("-- {}", command.get_handler().debug(&command));
    }
}