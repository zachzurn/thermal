use thermal_parser::thermal_file::parse_str;
use thermal_renderer::image_renderer::ImageRenderer;
use thermal_renderer::renderer::DebugProfile;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, WebGlRenderingContext as GL, HtmlCanvasElement, ImageData, WebGlShader, WebGlProgram};

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
        let result = render_pixels(&canvas, render.width, render.height, &render.bytes);
        return Ok(());
    } else {
        Err("Nothing to render")?;
    }

    Ok(())
}

pub fn render_pixels(
    canvas: &HtmlCanvasElement,
    width: u32,
    height: u32,
    pixel_data: &[u8],
) -> Result<(), String> {
    canvas.set_width(width);
    canvas.set_height(height);
    
    let gl: GL = canvas
        .get_context("webgl")
        .unwrap()
        .unwrap()
        .dyn_into()
        .unwrap();

    // Vertex shader source
    let vert_shader_src = r#"
        attribute vec2 a_position;
        attribute vec2 a_tex_coord;
        varying vec2 v_tex_coord;
        void main() {
            gl_Position = vec4(a_position, 0.0, 1.0);
            v_tex_coord = a_tex_coord;
        }
    "#;

    // Fragment shader source
    let frag_shader_src = r#"
        precision mediump float;
        varying vec2 v_tex_coord;
        uniform sampler2D u_texture;
        void main() {
            gl_FragColor = texture2D(u_texture, v_tex_coord);
        }
    "#;

    // Compile shaders and link program
    let vert_shader = compile_shader(&gl, GL::VERTEX_SHADER, vert_shader_src)?;
    let frag_shader = compile_shader(&gl, GL::FRAGMENT_SHADER, frag_shader_src)?;
    let program = link_program(&gl, &vert_shader, &frag_shader)?;

    gl.use_program(Some(&program));

    // Set up a full-screen quad
    let vertices: [f32; 16] = [
        // Positions   // Texture coords
        -1.0, -1.0,    0.0, 1.0,
        1.0, -1.0,    1.0, 1.0,
        -1.0,  1.0,    0.0, 0.0,
        1.0,  1.0,    1.0, 0.0,
    ];

    let buffer = gl.create_buffer().ok_or("Failed to create buffer")?;
    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer));

    unsafe {
        let vertices_array_buf_view = js_sys::Float32Array::view(&vertices);
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vertices_array_buf_view, GL::STATIC_DRAW);
    }

    let position_attrib = gl.get_attrib_location(&program, "a_position") as u32;
    let tex_coord_attrib = gl.get_attrib_location(&program, "a_tex_coord") as u32;

    gl.enable_vertex_attrib_array(position_attrib);
    gl.vertex_attrib_pointer_with_i32(position_attrib, 2, GL::FLOAT, false, 16, 0);

    gl.enable_vertex_attrib_array(tex_coord_attrib);
    gl.vertex_attrib_pointer_with_i32(tex_coord_attrib, 2, GL::FLOAT, false, 16, 8);

    // Create texture and upload pixel data
    let texture = gl.create_texture().ok_or("Failed to create texture")?;
    gl.bind_texture(GL::TEXTURE_2D, Some(&texture));

    // Configure texture parameters
    gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_S, GL::CLAMP_TO_EDGE as i32);
    gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_T, GL::CLAMP_TO_EDGE as i32);
    gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::NEAREST as i32);
    gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MAG_FILTER, GL::NEAREST as i32);

    // Upload pixel data to texture
    let data_len = (width * height * 4) as usize;
    let mut rgba_data = Vec::with_capacity(data_len);
    for chunk in pixel_data.chunks(3) {
        rgba_data.extend_from_slice(chunk); // r, g, b
        rgba_data.push(255); // a
    }

    gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
        GL::TEXTURE_2D,
        0,
        GL::RGBA as i32,
        width as i32,
        height as i32,
        0,
        GL::RGBA,
        GL::UNSIGNED_BYTE,
        Some(&rgba_data),
    ).map_err(|e| format!("Failed to upload texture data: {:?}", e))?;

    // Draw
    gl.draw_arrays(GL::TRIANGLE_STRIP, 0, 4);

    Ok(())
}

// Helper to compile shaders
fn compile_shader(gl: &GL, shader_type: u32, source: &str) -> Result<WebGlShader, String> {
    let shader = gl.create_shader(shader_type).ok_or("Unable to create shader object")?;
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if gl.get_shader_parameter(&shader, GL::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(gl.get_shader_info_log(&shader).unwrap_or("Unknown error creating shader".to_string()))
    }
}

// Helper to link program
fn link_program(gl: &GL, vert_shader: &WebGlShader, frag_shader: &WebGlShader) -> Result<WebGlProgram, String> {
    let program = gl.create_program().ok_or("Unable to create shader program")?;
    gl.attach_shader(&program, vert_shader);
    gl.attach_shader(&program, frag_shader);
    gl.link_program(&program);

    if gl.get_program_parameter(&program, GL::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(gl.get_program_info_log(&program).unwrap_or("Unknown error creating program".to_string()))
    }
}