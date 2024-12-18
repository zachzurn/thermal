//! HTML Renderer
//!
//! The HTML Renderer renders receipts to html.
//!
//! All CSS and images are embedded into the one file.
//!
//! Page mode is a special mode that generates a separate image.
//! Any page mode commands are rendered to an image using the
//! same thermal_image module that Image Renderer uses.
//!
//! Images are embedded into the html file by using the
//! Base64 url format.
//!
//! Barcodes and Qr Codes are rendered to SVG that is
//! inlined into the html content.

mod thermal_html;

use crate::html_renderer::thermal_html::{encode_html_image, graphics_to_svg, spans_to_html};
use crate::image_renderer::thermal_image::ThermalImage;
use crate::renderer::{DebugProfile, OutputRenderer, RenderOutput, Renderer};
use thermal_parser::context::{Context, PrintDirection, Rotation, TextJustify};
use thermal_parser::graphics::{Image, ImageFlow, VectorGraphic};
use thermal_parser::text::TextSpan;

static TEMPLATE: &str = include_str!("../../resources/templates/thermal.html");

/// ReceiptHtml is the main output for the html renderer
/// the content contains all html necessary to render the
/// receipt.
pub struct ReceiptHtml {
    pub content: String,
}

pub struct HtmlRenderer {
    pub last_y: u32,
    pub content: Vec<String>,
    pub template: String,
    pub page_image: ThermalImage,
    pub debug_profile: DebugProfile,
}

pub struct HtmlRow {
    y: u32,
    height: u32,
    content: String,
}

impl HtmlRow {
    pub fn empty() -> Self {
        Self {
            y: 0,
            height: 0,
            content: "".to_string(),
        }
    }
}

impl HtmlRenderer {
    pub fn new() -> Self {
        Self {
            last_y: 0,
            content: vec![],
            template: TEMPLATE.to_string(),
            page_image: ThermalImage::new(0),
            debug_profile: DebugProfile::default(),
        }
    }

    /// This is the normal way to render bytes to an html
    pub fn render(
        bytes: &Vec<u8>,
        debug_profile: Option<DebugProfile>,
    ) -> RenderOutput<ReceiptHtml> {
        let mut child_renderer: Box<dyn OutputRenderer<_>> = Box::new(HtmlRenderer::new());
        let mut renderer = Renderer::new(
            &mut child_renderer,
            debug_profile.unwrap_or(DebugProfile::default()),
        );
        renderer.render(bytes)
    }

    fn push_row(&mut self, row: HtmlRow) {
        self.content.push(format!(
            "<p style='height: {}px; margin-top: {}px'>{}</p>",
            row.height,
            row.y.saturating_sub(self.last_y),
            row.content
        ));
        self.last_y = row.y + row.height;
    }
}

impl OutputRenderer<ReceiptHtml> for HtmlRenderer {
    fn set_debug_profile(&mut self, profile: DebugProfile) {
        self.debug_profile = profile;
    }

    fn begin_render(&mut self, context: &mut Context) {
        self.page_image.debug_profile = self.debug_profile;
        self.page_image.paper_color = context.graphics.render_colors.paper_color;

        //Initialize image area for page mode
        self.page_image.set_width(0);

        //Page images should not auto grow in either direction
        //Normally only the width is locked down, but for page mode
        //We want to lock down the height as well
        self.page_image.auto_grow = false;

        //We keep track of the last y so that we can render things
        //from top to bottom in the html flow instead of having
        //every element be position absolute
        self.last_y = context.get_y();

        self.content.clear();
        self.push_row(HtmlRow {
            y: 0,
            height: context.get_y(),
            content: "".to_string(),
        })
    }

    fn page_begin(&mut self, _context: &mut Context) {
        self.page_image.set_width(0);
    }

    fn page_area_changed(
        &mut self,
        _context: &mut Context,
        rotation: Rotation,
        width: u32,
        height: u32,
    ) {
        let img = &mut self.page_image;

        match rotation {
            Rotation::R90 => img.rotate_90(),
            Rotation::R180 => img.rotate_180(),
            Rotation::R270 => img.rotate_270(),
            _ => {}
        }

        if width > self.page_image.width {
            self.page_image.expand_to_width(width)
        }
        if height > self.page_image.get_height() {
            self.page_image.expand_to_height(height)
        }
    }

    fn render_page(&mut self, context: &mut Context) {
        let rotation_to_standard = context.page_mode.calculate_directional_rotation(
            &context.page_mode.direction,
            &PrintDirection::TopLeft2Right,
        );

        //Rotate to standard direction
        match rotation_to_standard {
            Rotation::R90 => self.page_image.rotate_90(),
            Rotation::R180 => self.page_image.rotate_180(),
            Rotation::R270 => self.page_image.rotate_270(),
            _ => {}
        }

        let (w, h, pixels) = self.page_image.copy();

        //Rotate back to how it was
        let rotation_to_previous = context.page_mode.calculate_directional_rotation(
            &PrintDirection::TopLeft2Right,
            &context.page_mode.direction,
        );

        match rotation_to_previous {
            Rotation::R90 => self.page_image.rotate_90(),
            Rotation::R180 => self.page_image.rotate_180(),
            Rotation::R270 => self.page_image.rotate_270(),
            _ => {}
        }

        let image = thermal_parser::graphics::Image {
            pixels,
            x: context.graphics.render_area.x,
            y: context.graphics.render_area.y,
            w,
            h,
            flow: ImageFlow::Block,
            upside_down: false,
        };

        self.push_row(encode_html_image(&image));
    }

    fn render_graphics(&mut self, context: &mut Context, graphics: &Vec<VectorGraphic>) {
        if context.page_mode.enabled {
            for graphic in graphics {
                match graphic {
                    VectorGraphic::Rectangle(rectangle) => {
                        self.page_image.put_rect(rectangle, &context.text.color);
                    }
                }
            }
        } else {
            self.push_row(graphics_to_svg(graphics));
        }
    }

    fn render_image(&mut self, context: &mut Context, image: &Image) {
        if context.page_mode.enabled {
            self.page_image.put_render_img(image);
        } else {
            self.push_row(encode_html_image(image));
        }
    }

    fn render_text(
        &mut self,
        context: &mut Context,
        spans: &Vec<TextSpan>,
        x_offset: u32,
        max_height: u32,
        _text_justify: TextJustify,
    ) {
        if context.page_mode.enabled {
            for span in spans {
                if let Some(_) = &span.dimensions {
                    self.page_image.render_span(x_offset, max_height, span);
                }
            }
        } else {
            self.push_row(spans_to_html(
                spans,
                x_offset,
                max_height,
                0.78,
                &context.graphics.render_colors,
            ));
        }
    }

    fn end_render(&mut self, context: &mut Context) -> ReceiptHtml {
        let padding_bottom = context.get_y().saturating_sub(self.last_y);

        let content = self
            .template
            .replace(
                "{{color-0}}",
                &*context.graphics.render_colors.paper_color.as_hex(),
            )
            .replace(
                "{{color-1}}",
                &*context.graphics.render_colors.color_1.as_hex(),
            )
            .replace(
                "{{color-2}}",
                &*context.graphics.render_colors.color_2.as_hex(),
            )
            .replace(
                "{{color-3}}",
                &*context.graphics.render_colors.color_3.as_hex(),
            )
            .replace("{{content}}", &self.content.join(""))
            .replace(
                "{{receipt-style}}",
                &*format!(
                    "width: {}px; padding-left: {}px; padding-right: {}px; padding-bottom: {}px;",
                    context.graphics.paper_area.w,
                    context.graphics.paper_area.x,
                    context.graphics.paper_area.y,
                    padding_bottom
                ),
            );

        ReceiptHtml { content }
    }
}
