use png::BitDepth;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use thermal_parser::thermal_file::parse_str;
use thermal_renderer::html_renderer::HtmlRenderer;
// use thermal_renderer::html_renderer::HtmlRenderer;
use thermal_renderer::image_renderer::ImageRenderer;

#[test]
fn typography() {
    test_sample("typography", "thermal")
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
fn barcodes() {
    test_sample("barcodes", "thermal")
}

#[test]
fn thick_barcode() {
    test_sample("thick_barcode", "bin")
}

fn test_sample(name: &str, ext: &str) {
    let sample_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("sample_files")
        .join("in")
        .join(format!("{}.{}", name, ext));

    let img_out = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("sample_files")
        .join("out")
        .join("img")
        .join(format!("{}.{}", name, ext));

    let html_out = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("sample_files")
        .join("out")
        .join("html")
        .join(format!("{}.{}", name, ext));

    let bytes = if ext == "thermal" {
        let text = std::fs::read_to_string(sample_file.to_str().unwrap()).unwrap();
        parse_str(&text)
    } else {
        std::fs::read(sample_file.to_str().unwrap()).unwrap()
    };

    render_image(
        &bytes,
        format!("{}.png", img_out.to_str().unwrap().to_string()),
        name.to_string(),
    );
    render_html(
        &bytes,
        format!("{}.html", html_out.to_str().unwrap().to_string()),
        name.to_string(),
    );
}

fn render_html(bytes: &Vec<u8>, out_path: String, name: String) {
    let renders = HtmlRenderer::render(bytes);

    if let Some(render) = renders.output.first() {
        let path = Path::new(&out_path);
        let mut file = File::create(path).unwrap();
        file.write_all(render.content.as_bytes())
            .expect("Can't write output html");
    } else {
        assert!(false, "No image generated from renderer.");
    }

    let errors = renders.errors;

    if errors.len() > 0 {
        println!("Errors found for test file {}:", name);
        for error in errors {
            println!("{:?}", error);
        }
        assert!(false, "There were errors when rendering html.");
    }
}

fn render_image(bytes: &Vec<u8>, out_path: String, name: String) {
    let renders = ImageRenderer::render(bytes);

    if let Some(render) = renders.output.first() {
        save_image(&render.bytes, render.width, render.height, out_path);
    } else {
        assert!(false, "No image generated from renderer.");
    }

    let errors = renders.errors;

    if errors.len() > 0 {
        println!("Errors found for test file {}:", name);
        for error in errors {
            println!("{:?}", error);
        }
        assert!(false, "There were errors when rendering an image.");
    }
}

fn save_image(bytes: &Vec<u8>, width: u32, height: u32, out_path: String) {
    if bytes.len() == 0 || width == 0 || height == 0 {
        assert!(false, "No image generated from render.");
        return;
    }

    let path = Path::new(&out_path);
    let file = File::create(path).unwrap();
    let ref mut writer = BufWriter::new(file);
    let mut encoder = png::Encoder::new(writer, width, height);
    encoder.set_color(png::ColorType::Grayscale);
    encoder.set_depth(BitDepth::Eight);

    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(bytes).unwrap(); // Save
}
