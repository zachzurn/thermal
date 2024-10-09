use std::path::PathBuf;

use base64::engine::general_purpose;
use base64::Engine;
use png::{ColorType, Encoder};

use thermal_parser::command::DeviceCommand;
use thermal_parser::context::{Context, TextJustify};
use thermal_parser::graphics::{Image, TextSpan, VectorGraphic};

static TEMPLATE: &str = include_str!("../../resources/templates/thermal.html");

pub struct HtmlRenderer {
    pub out_path: String,
    pub content: Vec<String>,
    pub template: String,
    pub font_size_pixels: u32,
    pub receipt_width_pixels: u32,
    pub receipt_margin_left_pixels: u32,
    pub receipt_margin_right_pixels: u32,
    pub pixel_scale_ratio: f32,
    pub current_justify: TextJustify,
    pub gfx_x: u32,
    pub gfx_y: u32,
    pub gfx_w: u32,
    pub gfx_h: u32,
    pub gfx_svg: Vec<String>,
}

impl HtmlRenderer {
    pub fn new(out_path: String) -> Self {
        Self {
            out_path,
            content: vec![],
            template: TEMPLATE.to_string(),
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
    fn page_mode_supported() -> bool {
        false
    }

    fn begin_render(&mut self, context: &mut Context) {
        self.content.clear();
        let dpi = 152.0;
        self.pixel_scale_ratio = dpi / context.graphics.dots_per_inch as f32;
        self.font_size_pixels = (context.text.font_size as f32 * 1.63) as u32;
        self.receipt_width_pixels = context.graphics.paper_area.w;
        self.receipt_margin_left_pixels = context.graphics.paper_area.x;
        self.receipt_margin_right_pixels = context.graphics.paper_area.y;
        self.start_container(context);
    }

    fn render_graphics(&mut self, context: &mut Context, graphics: &Vec<VectorGraphic>) {
        self.end_container();
        self.start_container(context);
        let mut gfx_w = 0;
        let mut gfx_h = 0;
        let mut gfx_svg: Vec<String> = vec![];

        //draw graphics
        for graphic in graphics {
            match graphic {
                VectorGraphic::Rectangle(rectangle) => {
                    gfx_w = gfx_w.max(rectangle.x + rectangle.w);
                    gfx_h = gfx_h.max(rectangle.y + rectangle.h);

                    gfx_svg.push(format!(
                        "<rect width='{}' height='{}' x='{}' y='{}' fill='black' />",
                        rectangle.w, rectangle.h, rectangle.x, rectangle.y
                    ));
                }
            }
        }

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

    fn render_image(&mut self, context: &mut Context, image: &Image) {
        self.end_container();
        self.start_container(context);

        self.content.push(self.encode_html_image(image));

        self.end_container();
        self.start_container(context);
    }

    fn collect_text(&mut self, context: &mut Context, text: TextSpan) {
        self.maybe_start_container(context);
        let mut class_list = vec![];

        if text.bold {
            class_list.push("b");
        }

        if text.italic {
            class_list.push("i");
        }

        if text.strikethrough == 1 {
            class_list.push("s");
        }

        if text.strikethrough == 2 {
            class_list.push("sd");
        }

        if text.stretch_width > 1f32 {
            class_list.push("dw");
        }

        if text.stretch_height > 1f32 {
            class_list.push("dh");
        }

        if text.inverted {
            class_list.push("in");
        }

        if text.underline > 0 {
            class_list.push("u");
        }

        if text.underline > 1 {
            class_list.push("ud");
        }

        if text.upside_down == true {
            class_list.push("upd");
        }

        let css_class = class_list.join(" ");
        let br_text = text
            .text
            .replace("\n", &*format!("</span><br/><span class='{}'>", css_class));

        self.content
            .push(format!("<span class='{}'>{}</span>", css_class, br_text))
    }

    fn render_text(&mut self, _context: &mut Context, _layout: TextLayout) {}

    fn device_command(&mut self, _context: &mut Context, _command: &DeviceCommand) {}

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

    fn encode_html_image(&self, image: &Image) -> String {
        // Create a buffer to hold the PNG image data
        let mut png_data: Vec<u8> = Vec::new();

        // Create a PNG encoder with the specified width, height, and color type
        let mut encoder = Encoder::new(&mut png_data, image.w, image.h);
        encoder.set_color(ColorType::Grayscale);
        encoder.set_depth(png::BitDepth::Eight);

        // Write the PNG header and the image data
        let mut writer = encoder.write_header().expect("Failed to write PNG header");
        writer
            .write_image_data(&image.as_grayscale())
            .expect("Failed to write PNG image data");

        writer.finish().expect("Error encoding png");

        // Calculate the base64 representation of the PNG image
        let base64_encoded_image = general_purpose::STANDARD_NO_PAD.encode(&png_data);

        // Print or use the base64_encoded_image as needed
        format!(
            "<img width='{}' src='data:image/png;base64, {}' />",
            (image.w as f32 * self.pixel_scale_ratio) as u32,
            base64_encoded_image
        )
    }
}
