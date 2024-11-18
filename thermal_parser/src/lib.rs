use crate::command::Command;

pub mod command;
pub mod command_sets;
pub mod commands;
pub mod constants;
pub mod context;
pub mod decoder;
pub mod graphics;
pub mod parser;
pub mod subcommands;
pub mod thermal_file;
pub mod utils;
pub mod util;
pub mod text;

pub fn parse_esc_pos(bytes: &Vec<u8>) -> Vec<Command> {
    parser::Parser::new(command_sets::esc_pos::new()).parse_bytes(bytes)
}
