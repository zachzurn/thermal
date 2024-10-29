use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use thermal_parser::thermal_file::{cmds_to_thermal, parse_str, try_const, try_string};
use thermal_parser::{context::*, parse_esc_pos};

#[test]
fn code_pages() {
    test_sample("code_pages", "thermal")
}

#[test]
fn issuing_receipts() {
    test_sample("issuing_receipts", "thermal")
}

#[test]
fn page_mode() {
    test_sample("page_mode", "thermal")
}

#[test]
fn print_graphics() {
    test_sample("print_graphics", "thermal")
}

#[test]
fn receipt_with_barcode() {
    test_sample("receipt_with_barcode", "thermal")
}

#[test]
fn gs_images_column() {
    test_sample("gs_images_column", "bin")
}

#[test]
fn gs_images_raster() {
    test_sample("gs_images_raster", "bin")
}

#[test]
fn test_receipt_1() {
    test_sample("test_receipt_1", "bin")
}

#[test]
fn test_receipt_2() {
    test_sample("test_receipt_2", "bin")
}

#[test]
fn test_receipt_3() {
    test_sample("test_receipt_3", "bin")
}

#[test]
fn test_receipt_4() {
    test_sample("test_receipt_4", "bin")
}

#[test]
fn thick_barcode() {
    test_sample("thick_barcode", "bin")
}

#[test]
fn discount() {
    test_sample("discount", "bin")
}

#[test]
fn discount_convert() {
    convert_binary_to_thermal("discount")
}

fn test_sample(name: &str, ext: &str) {
    let sample_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("sample_files")
        .join("in")
        .join(format!("{}.{}", name, ext));

    let bytes = if ext == "thermal" {
        let text = std::fs::read_to_string(sample_file.to_str().unwrap()).unwrap();
        parse_str(&text)
    } else {
        std::fs::read(sample_file.to_str().unwrap()).unwrap()
    };

    parse(&bytes, true);
}

fn parse(bytes: &Vec<u8>, debug: bool) {
    let context = Context::new();

    let commands = parse_esc_pos(bytes);

    for cmd in commands {
        if debug {
            cmd.handler.debug(&cmd, &context);
        }
    }
}

// Utility function for converting bin files
// into the more readable format. Ideally all
// tests are in the .thermal format
fn convert_binary_to_thermal(name: &str) {
    let binary_filepath = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("sample_files")
        .join("in")
        .join(format!("{}.bin", name));

    let thermal_filepath = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("sample_files")
        .join("in")
        .join(format!("{}.thermal", name));

    let original_bytes = std::fs::read(binary_filepath.to_str().unwrap()).unwrap();
    let parsed_commands = parse_esc_pos(&original_bytes);
    let thermal_file = cmds_to_thermal(&parsed_commands);
    let new_bytes = parse_str(&thermal_file);

    for i in 0..original_bytes.len() {
        if new_bytes[i] != original_bytes[i] {
            let look_back = 15;
            let look_forward = 10;

            if i > look_back && i + look_forward <= original_bytes.len() {
                let mut debug_original = String::new();
                let mut debug_new = String::new();
                for j in i - look_back..i + look_forward + 1 {
                    debug_original.push_str(&*try_const(&original_bytes[j]));
                    debug_original.push(' ');

                    debug_new.push_str(&*try_const(&new_bytes[j]));
                    debug_new.push(' ');
                }

                println!("MISMATCH AT BYTE {}. See below for backtrace.", i);
                println!("ORIG: {}", &debug_original);
                println!("NEW:  {}", &debug_new);

                println!("{}", thermal_file);

                panic!("Bytes do not match at byte {}", i);
            }
        }
    }

    let mut file = File::create(thermal_filepath).unwrap();
    file.write_all(thermal_file.as_bytes())
        .expect("Can't write output to thermal file");
}
