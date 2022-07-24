use std::path::PathBuf;
use thermal_parser::{context::*, command::Command};

#[test]
fn it_parses_column_format() {
    test_binary_file("test_gs_images_column.bin", true);
}

#[test]
fn it_parses_raster_format() {
    test_binary_file("test_gs_images_raster.bin", true);
}

#[test]
fn it_parses_text_barcodes_code2d() {
    test_binary_file("test_receipt_4.bin", true);
}

#[test]
fn it_parses_test_1() {
    test_binary_file("test_receipt_1.bin", true);
}

#[test]
fn it_parses_test_2() {
    test_binary_file("test_receipt_2.bin", true);
}

#[test]
fn it_parses_test_3() {
    test_binary_file("test_receipt_3.bin", true);
}


fn test_binary_file(filename: &str, debug: bool){
    let bytes = std::fs::read(get_test_bin(filename)).unwrap();
    let context = Context::new();

    let on_new_command = move |cmd: Command| {
        if debug { println!("{}", cmd.handler.debug(&cmd, &context)) };
    };
    let mut command_parser = thermal_parser::new_esc_pos_parser(Box::from(on_new_command));
    command_parser.parse_bytes(&bytes);
}

fn get_test_bin(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("resources").join("test").join(name)
}


// command.handler.apply_context(&command, &mut context);
//
// if let Some(gfx) = command.handler.get_graphics(&command, &context){
//     match gfx {
//         GraphicsCommand::Code2D(_qr) => todo!(),
//         GraphicsCommand::Barcode(br) => {
//             print!("{:?}", br.text);
//         },
//         GraphicsCommand::Image(img) => {
//             let filepath = format!("test/gfx{:?}.pbm", context.graphics.graphics_count);
//             if let Ok(_) =  std::fs::write(filepath, img.as_pbm()) {}
//             context.graphics.graphics_count += 1;
//         },
//         _ => {}
//     }
// }
//
// if let Some(text) = command.handler.get_text(&command, &context){ print!("{}", text) }
//
// //Not going to be implemented but if the command wants to transmit data it can implement this
// if let Some(_return_bytes) = command.handler.transmit(&command, &context){};
