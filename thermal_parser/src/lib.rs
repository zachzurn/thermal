pub mod constants;
pub mod command;
pub mod commands;
pub mod subcommands;
pub mod command_sets;
pub mod graphics;
pub mod context;
pub mod parser;

pub fn new_esc_pos_parser() -> parser::Parser {
    parser::Parser::new(command_sets::esc_pos::new())
}