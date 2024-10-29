use std::path::PathBuf;
use thermal_parser::thermal_file::{parse_str};
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
    test_sample("discount", "thermal")
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
            println!("{}", cmd.handler.debug(&cmd, &context));
        }
    }
}