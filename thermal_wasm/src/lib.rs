use thermal_parser::thermal_file::parse_str;
use thermal_renderer::image_renderer::ImageRenderer;
use thermal_renderer::renderer::DebugProfile;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn render_to_canvas(thermal_str: &str, canvas: JsValue) -> Result<(), JsValue> {
    let canvas: HtmlCanvasElement = canvas.dyn_into::<HtmlCanvasElement>()?;

    let bytes = parse_str(thermal_str);
    let renders = ImageRenderer::render(&bytes, Some(DebugProfile::default()));
    let _errors = renders.errors;

    if let Some(render) = renders.output.first() {
        canvas.set_width(render.width);
        canvas.set_height(render.height);

        let context = canvas
            .get_context("2d")?
            .ok_or("Failed to get 2D context")?
            .dyn_into::<CanvasRenderingContext2d>()?;

        let mut rgba = Vec::with_capacity(((render.width * render.height) * 4) as usize);

        for chunk in render.bytes.chunks(3) {
            rgba.push(chunk[0]);
            rgba.push(chunk[1]);
            rgba.push(chunk[2]);
            rgba.push(0xFF);
        }

        let clamped_buf: Clamped<&[u8]> = Clamped(&rgba);

        let image_data_temp =
            ImageData::new_with_u8_clamped_array_and_sh(clamped_buf, render.width, render.height)?;

        let result = context.put_image_data(&image_data_temp, 0.0, 0.0)?;

        result
    } else {
        Err("Nothing to render")?;
    }

    Ok(())
}
