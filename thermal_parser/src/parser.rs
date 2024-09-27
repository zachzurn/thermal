use crate::command::CommandType;
use crate::{command::Command, command_sets::*};
use std::mem;

pub struct Parser {
    cmd_set: CommandSet,
    match_depth: u8,
    command_matches: Vec<Command>,
    current_command: Option<Command>,
    current_command_is_default: bool,
    command_buffer: Vec<u8>,
    on_command_found: Box<dyn FnMut(Command)>,
}

impl Parser {
    pub fn new(cmd_set: CommandSet, on_command_found: Box<dyn FnMut(Command)>) -> Self {
        Self {
            cmd_set,
            match_depth: 0,
            command_matches: Vec::<Command>::new(),
            current_command_is_default: false,
            command_buffer: Vec::<u8>::new(),
            current_command: None,
            on_command_found,
        }
    }

    pub fn parse_bytes(&mut self, bytes: &Vec<u8>) {
        self.emit_command(self.cmd_set.begin_parsing.clone());

        for byte in bytes {
            self.parse(byte);
        }

        //emit the last command and reset the parser
        let mut new_cmd = None;
        mem::swap(&mut self.current_command, &mut new_cmd); //new_cmd has become the previous command after the swap

        if let Some(new_cmd_unwrapped) = new_cmd {
            self.emit_command(new_cmd_unwrapped);
        }

        self.emit_command(self.cmd_set.end_parsing.clone());

        self.match_depth = 0;
        self.command_buffer.clear();
        self.command_matches.clear();
        self.current_command_is_default = false;
    }

    fn emit_command(&mut self, mut cmd: Command) {
        if cmd.kind == CommandType::Subcommand {
            let command = &mut cmd;

            if let Some(subcommand) = command.handler.get_subcommand() {
                (self.on_command_found)(subcommand)
            }
        } else {
            (self.on_command_found)(cmd);
        }
    }

    fn parse(&mut self, byte: &u8) {
        //If a command is willing to accept bytes and it is not the
        // default command, we don't need to do any filtering
        if self.match_depth == 0 && !self.current_command_is_default {
            if let Some(cmd) = &mut self.current_command {
                if cmd.push(*byte) {
                    return;
                };
            }
        }

        //Keep track of the search in case we need to match for an unknown command
        self.command_buffer.push(*byte);

        //Look for matching commands
        let mut new_command_matches: Vec<Command> = Vec::with_capacity(0);
        let subset = if self.match_depth == 0 {
            &self.cmd_set.commands
        } else {
            &self.command_matches
        };

        for cmd in subset.iter() {
            if let Some(b) = cmd.commands.get(self.match_depth as usize) {
                if b == byte {
                    new_command_matches.push(cmd.clone())
                }
            }
        }

        self.command_matches = new_command_matches;

        //if the command subset has one match, we create a new command by cloning the command
        if self.command_matches.len() == 1 {
            if let Some(matched_command) = &mut self.command_matches.first() {
                //Here we make sure all command bytes are matched
                if matched_command.commands.len() - 1 != self.match_depth as usize {
                    self.match_depth += 1;
                    return;
                }

                self.current_command_is_default = false;
                self.command_buffer.clear();
                self.match_depth = 0;

                let mut new_cmd = Some(matched_command.clone());
                mem::swap(&mut self.current_command, &mut new_cmd); //new_cmd has become the previous command after the swap

                if let Some(new_cmd_unwrapped) = new_cmd {
                    self.emit_command(new_cmd_unwrapped);
                }
            }
            return;
        }

        //If the matched command set is empty we either make a new default command
        //or make a new unknown command or append to the last default command
        if self.command_matches.is_empty() {
            let mut new_cmd = None;

            if self.command_buffer.len() > 0
                && self
                    .cmd_set
                    .unknown
                    .commands
                    .contains(self.command_buffer.first().unwrap())
            {
                let mut unknown_command = self.cmd_set.unknown.clone();
                unknown_command.data = self.command_buffer.clone();
                unknown_command.data.push(*byte);
                new_cmd = Some(unknown_command);
            } else if self.current_command_is_default {
                if let Some(cmd) = &mut self.current_command {
                    cmd.push(*byte);
                }
            } else {
                let mut default_command = self.cmd_set.default.clone();
                default_command.push(*byte);
                new_cmd = Some(default_command);
            }

            self.command_buffer.clear();
            self.current_command_is_default = true;
            self.match_depth = 0;

            if new_cmd.is_some() {
                mem::swap(&mut self.current_command, &mut new_cmd);
                if new_cmd.is_some() {
                    //new_command has become the previous command after the swap
                    self.emit_command(new_cmd.unwrap());
                } else {
                    //Generally an unknown command at the start of the binary
                    self.emit_command(self.cmd_set.unknown.clone());
                }
                return;
            }

            return;
        }
        self.match_depth += 1;
    }
}
