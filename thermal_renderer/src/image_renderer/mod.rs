use crate::image_renderer::thermal_image::ThermalImage;
use crate::renderer::CommandRenderer;
use std::mem;
use thermal_parser::command::DeviceCommand;
use thermal_parser::context::{Context, PrintDirection, Rotation};
use thermal_parser::graphics::{Image, TextLayout, TextSpan, VectorGraphic};

pub mod thermal_image;

pub struct ImageRenderer {
    pub image: ThermalImage,
    pub page_image: ThermalImage,
    pub text_layout: Option<TextLayout>,
    pub out_path: String,
    out_count: u32,
    spans: Vec<TextSpan>,
}

impl ImageRenderer {
    pub fn new(out_path: String) -> Self {
        Self {
            image: ThermalImage::new(0),
            page_image: ThermalImage::new(0),
            text_layout: None,
            out_path,
            out_count: 0,
            spans: vec![],
        }
    }
}

impl CommandRenderer for ImageRenderer {
    fn begin_render(&mut self, context: &mut Context) {
        self.image.set_width(context.graphics.render_area.w);
        self.page_image.set_width(0);
        //Page images should not auto grow in either direction
        //Normally only the width is locked down, but for page mode
        //We want to lock down the height as well
        self.page_image.auto_grow = false;
    }

    fn page_mode_supported() -> bool {
        true
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

    fn page_end(&mut self, _context: &mut Context) {}

    fn page_print(&mut self, context: &mut Context) {
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

        self.image.put_pixels(
            context.graphics.render_area.x,
            context.graphics.render_area.y,
            w,
            h,
            pixels,
            false,
            false,
        );
    }

    fn graphics(&mut self, context: &mut Context, graphics: &Vec<VectorGraphic>) {
        let page = context.page_mode.enabled;

        for graphic in graphics {
            match graphic {
                VectorGraphic::Rectangle(rectangle) => {
                    if page {
                        self.page_image.put_rect(rectangle);
                    } else {
                        self.image.put_rect(rectangle);
                    }
                }
            }
        }
    }

    fn image(&mut self, context: &mut Context, image: &Image) {
        if context.page_mode.enabled {
            self.page_image.put_render_img(image);
        } else {
            self.image.put_render_img(image);
        }
    }

    fn text_span(&mut self, _context: &mut Context, text: TextSpan) {
        self.spans.push(text);
    }

    fn text_span_collect(&mut self, context: &mut Context, mut layout: TextLayout) {
        if self.spans.is_empty() {
            return;
        }
        
        mem::swap(&mut layout.spans, &mut self.spans);
        println!("Push layout {:?}", layout);
        let line_height = layout.line_height;
        let layout_x = layout.x;
        let layout_y = layout.y;

        let offset = if context.page_mode.enabled {
            self.page_image.draw_text(layout)
        } else {
            self.image.draw_text(layout)
        };

        println!(
            "Rendered Text Layout x{} y{} new X{} new y{} lh{}",
            layout_x, layout_y, offset.0, offset.1, line_height
        );
        context.set_x(offset.0);
        context.set_y(offset.1);
    }

    fn device_command(&mut self, _context: &mut Context, _command: &DeviceCommand) {}

    fn end_render(&mut self, context: &mut Context) {
        //Simulate post cut feeding
        self.image
            .add_top_margin(context.line_height_pixels() as u32 * 2);

        //Add in the left and right margin;
        self.image.expand_to_width(context.graphics.paper_area.w);

        //Feed to the y height to ensure we catch any cut advances
        self.image.expand_to_height(context.graphics.render_area.y);

        let out_path = self.unique_out_path();

        self.image.save_png(out_path);

        self.image.reset();
        context.graphics.render_area.x = 0;
        context.graphics.render_area.y = 0;
    }
}

impl ImageRenderer {
    fn unique_out_path(&mut self) -> String {
        self.out_count = self.out_count.wrapping_add(1);
        format!("{}.png", self.out_path.to_string())
    }
}
