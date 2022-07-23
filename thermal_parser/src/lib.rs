pub mod constants;
pub mod command;
pub mod commands;
pub mod subcommands;
pub mod command_sets;
pub mod graphics;
pub mod context;
pub mod parser;

pub fn new_esc_pos_parser(on_command_found: Box<dyn Fn(command::Command)>) -> parser::Parser {
    parser::Parser::new(command_sets::esc_pos::new(), on_command_found)
}
