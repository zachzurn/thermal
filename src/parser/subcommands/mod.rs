pub mod gs_code2d;
pub mod gs_graphics;

use std::rc::Rc;

use super::*;

#[derive(Clone)]
pub struct SubCommandHandler {
    commands: Rc<Vec<Command>>,
    subcommand: Option<Command>,
    is_large: bool,
    m: u8,
    subcommand_id: u8,
    capacity: u32,
    accept_data: bool,
    use_m: bool,
}

impl SubCommandHandler {
    fn detect_kind(&mut self) {
        for c in self.commands.iter() {
            if c.commands.contains(&self.subcommand_id) {
                self.subcommand = Some(c.clone());
                break;
            }
        }
    }
    fn detect_kind_use_m(&mut self) {
        for c in self.commands.iter() {
            if let Some(first_char) = c.commands.get(0) {
                if *first_char != self.m { continue; }
            }

            for (pos, byte) in c.commands.iter().enumerate() {
                if pos != 0 && self.subcommand_id == *byte {
                    self.subcommand = Some(c.clone());
                    break;
                }
            }
        }
    }
    fn parse_meta(&mut self, data: &[u8]) {
        let data_len = data.len();

        if data_len == 4 {
            self.capacity = u16::from_le_bytes([data[0], data[1]]) as u32;
            self.capacity -= 2;
            self.m = *data.get(2).unwrap();
            self.subcommand_id = *data.get(3).unwrap();
        }

        if data_len == 6 {
            self.capacity = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
            self.capacity -= 2;
            self.m = *data.get(4).unwrap();
            self.subcommand_id = *data.get(5).unwrap();
        }

        if self.use_m { self.detect_kind_use_m() } else { self.detect_kind() }
        self.accept_data = true;
    }
}

//We are proxying all command handler commands to the subcommand with the exception of parse
impl CommandHandler for SubCommandHandler {
    fn get_text(&self, command: &Command, context: &Context) -> Option<String> {
        if let Some(subcommand) = &self.subcommand {
            return subcommand.handler.get_text(command, context);
        }
        None
    }

    fn get_graphics(&self, command: &Command, context: &Context) -> Option<GraphicsCommand> {
        if let Some(subcommand) = &self.subcommand {
            return subcommand.handler.get_graphics(command, context);
        }
        None
    }

    fn apply_context(&self, command: &Command, context: &mut Context) {
        if let Some(subcommand) = &self.subcommand {
            subcommand.handler.apply_context(command, context)
        }
    }

    fn debug(&self, _command: &Command, _context: &Context) -> String {
        if let Some(subcommand) = &self.subcommand {
            return subcommand.name.to_string();
        }
        "".to_string()
    }

    fn push(&mut self, data: &mut Vec<u8>, byte: u8) -> bool {
        let data_len = data.len();

        if !self.accept_data {
            if self.is_large {
                if data_len < 6 {
                    data.push(byte);
                    return true;
                }
                self.parse_meta(&data[0..6]);
            } else {
                if data_len < 4 {
                    data.push(byte);
                    return true;
                }
                self.parse_meta(&data[0..4]);
            }
            data.clear();
        }

        //Accept data
        if data_len < (self.capacity as usize) {
            data.push(byte);
            return true;
        }

        false
    }
}

pub fn new(is_large: bool, use_m: bool, commands: Rc<Vec<Command>>) -> Box<SubCommandHandler> {
    Box::new(SubCommandHandler {
        commands,
        subcommand: None,
        is_large,
        m: 0,
        subcommand_id: 0,
        capacity: 0,
        accept_data: false,
        use_m,
    })
}

pub fn no_commands() -> Rc<Vec<Command>> {
    let all: Vec<Command> = vec![];
    Rc::new(all)
}