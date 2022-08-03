use std::path::PathBuf;
use thermal_parser::command::Command;
use thermal_parser::context::Context;
use thermal_renderer::image_renderer::ImageRenderer;
use thermal_renderer::renderer::CommandRenderer;

#[test]
fn it_renders_1(){
    it_renders("test_gs_images_raster.bin");
}

fn it_renders(filename: &str) {
    let bytes = std::fs::read(get_test_bin(filename)).unwrap();
    let mut context = Context::new();
    let mut image_renderer = ImageRenderer::new(format!("{}/{}/{}", env!("CARGO_MANIFEST_DIR"), "resources", "out"));

    let on_new_command = move |cmd: Command| {
        image_renderer.process_command(&mut context, &cmd);
    };

    let mut command_parser = thermal_parser::new_esc_pos_parser(Box::from(on_new_command));
    command_parser.parse_bytes(&bytes);
}

fn get_test_bin(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("resources").join("test").join(name)
}
