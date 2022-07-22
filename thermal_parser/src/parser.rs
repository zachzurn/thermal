use crate::{command::Command, command_sets::*};

pub struct Parser{
    cmd_set: CommandSet,
}

impl Parser {
    pub fn new(cmd_set: CommandSet) -> Self {
        Self {
            cmd_set
        }
    }

    //TODO implement a parsing method that can stream individual bytes instead of all at once
    pub fn parse_all(&self, bytes: &Vec<u8>) -> Vec<Command> {
        let mut match_depth = 0u8;
        let mut command_matches: Box<Vec<Command>> = Box::new(Vec::<Command>::new());
        let mut commands: Vec<Command> = vec![];
        let mut last_command_is_default = false;
        let mut command_buffer: Vec<u8> = vec![];

        for byte in bytes {

            //If a command is willing to accept bytes and it is not the
            // default command, we don't need to do any filtering
            if match_depth == 0 && !last_command_is_default {
                if let Some(cmd) = commands.last_mut() {
                    if cmd.push(*byte) { continue; };
                }
            }

            //Keep track of the search in case we need to match for an unknown command
            command_buffer.push(*byte);

            //Try to match a command
            command_matches = if match_depth == 0 {
                Self::match_commands(&byte, &match_depth, &self.cmd_set.commands)
            } else {
                Self::match_commands(&byte, &match_depth, &command_matches)
            };

            //if the command subset has one match, we create a new command by cloning the command
            if command_matches.len() == 1 {
                if let Some(matched_command) = command_matches.first() {
                    //Here we make sure all command bytes are matched
                    if matched_command.commands.len() - 1 != match_depth as usize {
                        match_depth += 1;
                        continue;
                    }
                    command_buffer.clear();
                    commands.push(matched_command.clone().to_owned());
                    last_command_is_default = false;
                }
                match_depth = 0;
                continue;
            }

            //If the matched command set is empty we either make a new default command
            //or make a new unknown command or append to the last default command
            if command_matches.is_empty() {
                if command_buffer.len() > 0 && self.cmd_set.unknown.commands.contains(command_buffer.first().unwrap()) {
                    let mut unknown_command = self.cmd_set.unknown.clone();
                    unknown_command.data = command_buffer.clone();
                    unknown_command.data.push(*byte);
                    commands.push(unknown_command);
                } else if last_command_is_default {
                    commands.last_mut().unwrap().push(*byte);
                } else {
                    let mut default_command = self.cmd_set.default.clone();
                    default_command.push(*byte);
                    commands.push(default_command);
                }
                command_buffer.clear();
                last_command_is_default = true;
                match_depth = 0;
                continue;
            }

            match_depth += 1;
        }
        commands
    }

    fn match_commands(byte: &u8, depth: &u8, command_set: &Box<Vec<Command>>) -> Box<Vec<Command>> {
        let mut current_command_set: Vec<Command> = Vec::with_capacity(0);

        for cmd in command_set.iter() {
            if let Some(b) = cmd.commands.get(*depth as usize) {
                if b == byte { current_command_set.push(cmd.clone()) }
            }
        }

        Box::from(current_command_set)
    }
}