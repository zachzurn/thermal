use crate::parser::Command;

fn subset_commands<'a>(byte: u8, depth: u8, command_set: Box<Vec<Command>>) -> Box<Vec<Command>>{
  let mut current_command_set: Vec<Command> = Vec::with_capacity(0);

  for cmd in command_set.iter() {
      match cmd.commands.get(depth as usize) {
          None => {}
          Some(b) => {
              if *b == byte { current_command_set.push(cmd.to_owned()) }
          } 
      }
  }

  Box::from(current_command_set)
}

/// 
pub struct CommandSet{
  pub commands: Box<Vec<Command>>, //list of supported commands
  pub default: Command //default command (normally a text command)
}


impl CommandSet {
  //TODO possibly implement a parsing method that can stream individual bytes instead of all at once
  pub fn parse(&self, bytes: &Vec<u8>) -> Vec<Command> {
    let mut depth= 0u8;
    let empty_command_subset: Vec<Command> = vec!();
    let mut command_subset: Box<Vec<Command>> = Box::from(empty_command_subset);
    let mut commands: Vec<Command> = vec![];
    let mut last_command_is_default = false;

    for byte in bytes {

      //If a command is willing to accept bytes and it is not the default command, we don't need to do any filtering
      if depth == 0 && !last_command_is_default {
        if let Some(cmd) = commands.last_mut() { if cmd.push(*byte) { continue; } }
      }
      
      command_subset = if depth == 0 { 
        subset_commands(*byte, depth, self.commands.to_owned())
      } else { 
        subset_commands(*byte, depth, command_subset) 
      };

      if command_subset.is_empty() {

        if last_command_is_default { 
          //We can safely push bytes if there are no matched commands and the last command is default
          commands.last_mut().unwrap().push(*byte); 
        } else {
          //Create a new default command
          let mut default_command = self.default.clone().to_owned();
          default_command.push(*byte);
          commands.push(default_command);
          last_command_is_default = true;
          depth = 0;
        }

        continue;
      }

      //if the command subset has one match, we create a new command by cloning the command
      if command_subset.len() == 1 {
        if let Some(matched_command) = command_subset.first() {
          //We do this to make sure all commands match instead of just the first  
          if matched_command.commands.len() -1 != depth as usize { depth += 1; continue; }
          commands.push(matched_command.clone().to_owned());
          last_command_is_default = false;
          depth = 0;
        }             
        continue;    
      }

      depth += 1;
    }
    commands
  }
}

pub mod esc_pos;