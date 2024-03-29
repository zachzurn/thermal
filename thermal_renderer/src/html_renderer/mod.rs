use crate::renderer::CommandRenderer;
use base64::engine::general_purpose;
use base64::Engine;
use png::{ColorType, Encoder};
use std::path::PathBuf;
use thermal_parser::command::DeviceCommand;
use thermal_parser::context::{Context, TextJustify, TextStrikethrough, TextUnderline};

pub struct HtmlRenderer {
    pub out_path: String,
    pub content: Vec<String>,
    pub template: String,
    pub font_size_pixels: usize,
    pub receipt_width_pixels: usize,
    pub receipt_margin_left_pixels: usize,
    pub receipt_margin_right_pixels: usize,
    pub pixel_scale_ratio: f32,
    pub current_justify: TextJustify,
    pub gfx_x: usize,
    pub gfx_y: usize,
    pub gfx_w: usize,
    pub gfx_h: usize,
    pub gfx_svg: Vec<String>,
}

impl HtmlRenderer {
    pub fn new(out_path: String) -> Self {
        let template_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("resources")
            .join("templates")
            .join("thermal.html");

        Self {
            out_path,
            content: vec![],
            template: std::fs::read_to_string(template_path).unwrap(),
            font_size_pixels: 0,
            receipt_width_pixels: 0,
            receipt_margin_left_pixels: 0,
            receipt_margin_right_pixels: 0,
            pixel_scale_ratio: 0.0,
            current_justify: TextJustify::Left,
            gfx_x: 0,
            gfx_y: 0,
            gfx_w: 0,
            gfx_h: 0,
            gfx_svg: vec![],
        }
    }
}

impl CommandRenderer for HtmlRenderer {
    fn begin_render(&mut self, context: &mut Context) {
        self.content.clear();
        let dpi = 152.0;
        self.pixel_scale_ratio = dpi / context.graphics.dots_per_inch as f32;
        self.font_size_pixels = (context.text.font_size as f32 * 1.63) as usize;
        self.receipt_width_pixels = (context.graphics.paper_width * dpi) as usize;
        self.receipt_margin_left_pixels = (context.graphics.margin_left * dpi) as usize;
        self.receipt_margin_right_pixels = (context.graphics.margin_left * dpi) as usize;
        self.start_container(context);
    }

    fn begin_graphics(&mut self, context: &mut Context) {
        self.end_container();
        self.start_container(context);
        self.gfx_x = context.graphics.x;
        self.gfx_y = context.graphics.y;
        self.gfx_w = 0;
        self.gfx_h = 0;
        self.gfx_svg = vec![];
    }

    fn draw_rect(&mut self, context: &mut Context, w: usize, h: usize) {
        let x = context.graphics.x - self.gfx_x;
        let y = context.graphics.y - self.gfx_y;

        self.gfx_w = self.gfx_w.max(x + w);
        self.gfx_h = self.gfx_h.max(y + h);

        self.gfx_svg.push(format!(
            "<rect width='{}' height='{}' x='{}' y='{}' fill='black' />",
            w, h, x, y
        ));
    }
    fn end_graphics(&mut self, context: &mut Context) {
        self.maybe_start_container(context);

        self.content.push(format!(
            "<svg width='{}' height='{}'>{}</svg>",
            self.gfx_w,
            self.gfx_h,
            self.gfx_svg.join("\n")
        ));

        self.end_container();
        self.start_container(context);
    }

    fn draw_image(&mut self, context: &mut Context, bytes: Vec<u8>, width: usize, height: usize) {
        self.end_container();
        self.start_container(context);

        self.content
            .push(self.encode_html_image(bytes, width as u32, height as u32));

        self.end_container();
        self.start_container(context);
    }

    fn draw_text(&mut self, context: &mut Context, text: String) {
        self.maybe_start_container(context);
        let mut class_list = vec![];

        if context.text.bold {
            class_list.push("b");
        }

        if context.text.italic {
            class_list.push("i");
        }

        if context.text.strikethrough == TextStrikethrough::On {
            class_list.push("s");
        }

        if context.text.strikethrough == TextStrikethrough::Double {
            class_list.push("sd");
        }

        if context.text.width_mult == 2 {
            class_list.push("dw");
        }

        if context.text.height_mult == 2 {
            class_list.push("dh");
        }

        if context.text.invert == true {
            class_list.push("in");
        }

        if context.text.underline == TextUnderline::On {
            class_list.push("u");
        }

        if context.text.underline == TextUnderline::Double {
            class_list.push("ud");
        }

        if context.text.upside_down == true {
            class_list.push("upd");
        }

        let css_class = class_list.join(" ");
        let br_text = text.replace("\n", &*format!("</span><br/><span class='{}'>", css_class));

        self.content
            .push(format!("<span class='{}'>{}</span>", css_class, br_text))
    }

    fn draw_device_command(&mut self, _context: &mut Context, _command: &DeviceCommand) {}

    fn end_render(&mut self, _context: &mut Context) {
        //Close the last container
        self.end_container();

        let out = self
            .template
            .replace("{{content}}", &self.content.join(""))
            .replace(
                "{{receipt-style}}",
                &*format!(
                    "max-width: {}px; padding-left: {}px; padding-right: {}px;",
                    self.receipt_width_pixels,
                    self.receipt_margin_left_pixels,
                    self.receipt_margin_right_pixels
                ),
            )
            .replace("{{font-size}}", &self.font_size_pixels.to_string());

        std::fs::write(PathBuf::from(format!("{}{}", self.out_path, ".html")), out)
            .expect("Invalid out path");
    }
}

impl HtmlRenderer {
    fn maybe_start_container(&mut self, context: &mut Context) {
        //No need if justification hasn't changed
        if self.current_justify == context.text.justify {
            return;
        }

        self.end_container();
        self.start_container(context);
    }

    fn start_container(&mut self, context: &mut Context) {
        self.current_justify = context.text.justify.clone();
        let css_class = match self.current_justify {
            TextJustify::Left => String::from("al"),
            TextJustify::Center => String::from("ac"),
            TextJustify::Right => String::from("ar"),
        };

        self.content
            .push(format!("<div class='cnt {}'>", css_class));
    }

    fn end_container(&mut self) {
        self.content.push(String::from("</div>"));
    }

    fn encode_html_image(&self, bytes: Vec<u8>, width: u32, height: u32) -> String {
        // Create a buffer to hold the PNG image data
        let mut png_data: Vec<u8> = Vec::new();

        // Create a PNG encoder with the specified width, height, and color type
        let mut encoder = Encoder::new(&mut png_data, width, height);
        encoder.set_color(ColorType::Grayscale);
        encoder.set_depth(png::BitDepth::Eight);

        // Write the PNG header and the image data
        let mut writer = encoder.write_header().expect("Failed to write PNG header");
        writer
            .write_image_data(&bytes)
            .expect("Failed to write PNG image data");

        writer.finish().expect("Error encoding png");

        // Calculate the base64 representation of the PNG image
        let base64_encoded_image = general_purpose::STANDARD_NO_PAD.encode(&png_data);

        // Print or use the base64_encoded_image as needed
        format!(
            "<img width='{}' src='data:image/png;base64, {}' />",
            (width as f32 * self.pixel_scale_ratio) as usize,
            base64_encoded_image
        )
    }
}
