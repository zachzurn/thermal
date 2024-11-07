use crate::context::Context;
use crate::{command::*, constants::*};

#[derive(Clone)]
struct Handler {
    capacity: u32,
}

impl CommandHandler for Handler {
    fn push(&mut self, command: &mut Vec<u8>, byte: u8) -> bool {
        if command.len() < 2 {
            command.push(byte);
            return true;
        }

        if command.len() == 2 {
            let pl = *command.get(0).unwrap();
            let ph = *command.get(1).unwrap();
            self.capacity = (pl as u32 + ph as u32 * 256) + 2;
            command.push(byte);
            return true;
        }

        if command.len() < self.capacity as usize {
            command.push(byte);
            return true;
        }

        false
    }

    fn apply_context(&self, command: &Command, context: &mut Context) {
        if command.data.len() < 3 {
            return;
        }

        let fnc = command.data.get(2).unwrap();

        match fnc {
            // Select character color, one param {m}
            48 => {
                let m = command.data.get(3).unwrap_or(&48u8);
                context.text.color = *context.graphics.render_colors.color_for_number(*m);
            }
            //Selects background color by {m}
            49 => {
                let m = command.data.get(3).unwrap_or(&48u8);
                context.text.background_color =
                    *context.graphics.render_colors.color_for_number(*m);
            }
            //shadow color {m} shadow effect {a}
            50 => {
                let m = command.data.get(3).unwrap_or(&48u8);
                let a = command.data.get(4).unwrap_or(&0u8);

                context.text.shadow_color = *context.graphics.render_colors.color_for_number(*m);
                context.text.shadow = match a {
                    0 | 48 => false,
                    _ => true,
                }
            }
            _ => {}
        }
    }
}

pub fn new() -> Command {
    Command::new(
        "Set Character Effects",
        vec![GS, '(' as u8, 'N' as u8],
        CommandType::Context,
        DataType::Custom,
        Box::new(Handler { capacity: 2 }),
    )
}
