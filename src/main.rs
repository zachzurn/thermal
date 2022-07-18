mod parser;

use std::fs;

use parser::context::Context;

use crate::parser::*;
use crate::parser::command_sets::esc_pos;
use crate::parser::graphics::*;
use std::collections::HashMap;

fn main() {
    let bytes = std::fs::read("test/test_receipt_2.bin").unwrap();
    
    let esc_pos = esc_pos::new();
    let commands = esc_pos.parse(&bytes);
    let mut image_storage: HashMap<u8, Image> = HashMap::new();
    
    let mut context = Context::new();

    let mut graphics_id = 0;

    for command in commands {
        match command.kind {
            CommandType::Graphics => {
                if let Some(gfx) = command.handler.get_graphics(&command, &context){
                    match gfx {
                        GraphicsCommand::Qrcode(_qr) => todo!(),
                        GraphicsCommand::Barcode(_br) => todo!(),
                        //Images can be stored or printed if the image has a storage_id we store it and do not print it
                        GraphicsCommand::Image(img) => {
                            if let Some(storage_id) = img.storage_id {
                                image_storage.insert(storage_id, img);
                                println!("Stored Image {}", storage_id);
                            } else {
                                let filepath = format!("test/gfx{:?}.pbm", graphics_id);
                                graphics_id += 1;
                                if let Ok(_) = fs::write(filepath, img.as_pbm()) {}
                            }
                        },
                        //ImageRefs point to a stored image. If no image is stored we do nothing
                        GraphicsCommand::ImageRef(imgref) => {
                            if let Some(img) = image_storage.get(&imgref.storage_id) {
                                let filepath = format!("test/{:?}.pbm", imgref.storage_id);
                                if let Ok(_) = fs::write(filepath, img.as_pbm()) {}
                            }
                        },
                        _ => {}
                    }
                }
            },
            CommandType::Context => {
                command.handler.apply_context(&command, &mut context)
            }
            _ => {},
        }

        println!("{}", command.handler.debug(&command, &context));

        graphics_id += 1;
    }
}