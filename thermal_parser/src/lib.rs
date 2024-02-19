pub mod command;
pub mod command_sets;
pub mod commands;
pub mod constants;
pub mod context;
pub mod graphics;
pub mod parser;
pub mod subcommands;
pub mod util;
pub mod utils;

pub fn new_esc_pos_parser(on_command_found: Box<dyn FnMut(command::Command)>) -> parser::Parser {
    parser::Parser::new(command_sets::esc_pos::new(), on_command_found)
}
