mod parser;

use std::fs;

use parser::context::Context;
use crate::parser::command_sets::esc_pos;
use crate::parser::graphics::*;

fn main() {
    let bytes = std::fs::read("test/test_receipt_2.bin").unwrap();
    
    let esc_pos = esc_pos::new();
    let commands = esc_pos.parse(&bytes);
    let mut context = Context::new();

    for command in commands {
        
        command.handler.apply_context(&command, &mut context);

        if let Some(gfx) = command.handler.get_graphics(&command, &context){
            match gfx {
                GraphicsCommand::Code2D(_qr) => todo!(),
                GraphicsCommand::Barcode(_br) => todo!(),
                GraphicsCommand::Image(img) => {
                    let filepath = format!("test/gfx{:?}.pbm", context.graphics.graphics_count);
                    if let Ok(_) = fs::write(filepath, img.as_pbm()) {}
                    context.graphics.graphics_count += 1;
                },
                _ => {}
            }
        }

        if let Some(text) = command.handler.get_text(&command, &context){ print!("{}", text) }

        //Not going to be implemented but if the command wants to transmit data it can implement this
        if let Some(_return_bytes) = command.handler.transmit(&command, &context){};

    }
}