use std::sync::Arc;

use crate::parser::Command;

fn subset_commands<'a>(byte: u8, depth: u8, command_set: Arc<Vec<Command>>) -> Arc<Vec<Command>>{
  let mut current_command_set: Vec<Command> = Vec::with_capacity(0);

  for cmd in command_set.iter() {
      match cmd.commands.get(depth as usize) {
          None => {}
          Some(b) => {
              if *b == byte { current_command_set.push(cmd.to_owned()) }
          } 
      }
  }

  Arc::from(current_command_set)
}

pub struct CommandSet{
  pub commands: Arc<Vec<Command>>,
  pub default: Command
}


impl CommandSet {
  pub fn parse_commands(&self, bytes: &Vec<u8>) -> Vec<Command> {
    let mut depth= 0u8;
    let empty_command_subset: Vec<Command> = vec!();
    let mut command_subset: Arc<Vec<Command>> = Arc::from(empty_command_subset);
    let mut commands: Vec<Command> = vec![];

    for byte in bytes {
        //get the command set that matches the depth and byte command
        command_subset = if depth == 0 { 
          subset_commands(*byte, depth, self.commands.to_owned())
        } else { 
          subset_commands(*byte, depth, command_subset) 
        };


        //if the command subset is empty we push bytes to the last command or generate the default command
        if command_subset.is_empty() {
            if let Some(cmd) = commands.last_mut() { 
              if cmd.push(*byte) { 
                depth = 0;
                continue; 
              }
            }

            let mut default_command = self.default.clone().to_owned();
            default_command.push(*byte);
            commands.push(default_command);

            depth = 0;
            continue;
        }

        //if the command subset has one match, we create a new command by cloning the command
        if command_subset.len() == 1 {
            if let Some(matched_command) = command_subset.first() {
              //We do this to make sure all commands match instead of just the first  
              if matched_command.commands.len() -1 != depth as usize {
                    depth += 1;
                    continue;
              }

              commands.push(matched_command.clone().to_owned());
              depth = 0;
            }             
            continue;    
        }

        //We keep looking for command matches
        depth += 1;
    }
    commands
  }
}

pub mod esc_pos;