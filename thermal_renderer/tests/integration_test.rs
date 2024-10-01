use std::path::PathBuf;
use thermal_parser::command::Command;
use thermal_parser::context::Context;
use thermal_parser::thermal_file::parse_str;
use thermal_renderer::html_renderer::HtmlRenderer;
use thermal_renderer::image_renderer::ImageRenderer;
use thermal_renderer::renderer::CommandRenderer;

#[test]
fn scale_test() {
    it_renders("scale_test.bin");
}

#[test]
fn gs_image_column() {
    it_renders("test_gs_images_column.bin");
}

#[test]
fn gs_image_raster() {
    it_renders("test_gs_images_raster.bin");
}

#[test]
fn receipt_1() {
    it_renders("test_receipt_1.bin");
}

#[test]
fn receipt_2() {
    it_renders("test_receipt_2.bin");
}

#[test]
fn receipt_3() {
    it_renders("test_receipt_3.bin");
}

#[test]
fn receipt_4() {
    it_renders("test_receipt_4.bin");
}

#[test]
fn thick_barcode() {
    it_renders("thick_barcode.bin");
}

#[test]
fn corrupt_start_of_binary() {
    it_renders("corrupt_start_of_binary.bin");
}

#[test]
fn receipt_with_barcode() {
    it_renders("receipt_with_barcode.thermal");
}

#[test]
fn bad_image() {
    it_renders("bad_image.thermal");
}

#[test]
fn print_graphics() {
    it_renders("print_graphics.thermal");
}

#[test]
fn issuing_receipts() {
    it_renders("issuing_receipts.thermal");
}

#[test]
fn code_pages() {
    it_renders("code_pages.thermal")
}

fn it_renders(filename: &str) {
    it_renders_image(filename);
    it_renders_html(filename);
}

fn it_renders_html(filename: &str) {
    let out = format!(
        "{}/{}/{}/{}",
        env!("CARGO_MANIFEST_DIR"),
        "resources",
        "out",
        filename
    );

    let bytes = if filename.ends_with(".thermal") {
        let text = std::fs::read_to_string(get_test_bin(filename)).unwrap();
        parse_str(&text)
    } else {
        std::fs::read(get_test_bin(filename)).unwrap()
    };

    let mut html_renderer = HtmlRenderer::new(out);

    let mut context = Context::new();

    let on_new_command = move |cmd: Command| {
        html_renderer.process_command(&mut context, &cmd);
    };

    let mut command_parser = thermal_parser::new_esc_pos_parser(Box::from(on_new_command));
    command_parser.parse_bytes(&bytes);
}

fn it_renders_image(filename: &str) {
    let out = format!(
        "{}/{}/{}/{}",
        env!("CARGO_MANIFEST_DIR"),
        "resources",
        "out",
        filename
    );

    let bytes = if filename.ends_with(".thermal") {
        let text = std::fs::read_to_string(get_test_bin(filename)).unwrap();
        parse_str(&text)
    } else {
        std::fs::read(get_test_bin(filename)).unwrap()
    };

    let mut image_renderer = ImageRenderer::new(out);

    let mut context = Context::new();

    let on_new_command = move |cmd: Command| {
        image_renderer.process_command(&mut context, &cmd);
    };

    let mut command_parser = thermal_parser::new_esc_pos_parser(Box::from(on_new_command));
    command_parser.parse_bytes(&bytes);
}

fn get_test_bin(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("resources")
        .join("test")
        .join(name)
}
