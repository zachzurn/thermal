use crate::image_renderer::thermal_image::ThermalImage;
use crate::renderer::{OutputRenderer, RenderOutput, Renderer};
use thermal_parser::command::DeviceCommand;
use thermal_parser::context::{Context, PrintDirection, Rotation, TextJustify};
use thermal_parser::graphics::{Image, VectorGraphic};
use thermal_parser::text::TextSpan;

pub mod thermal_image;

pub struct ImageRenderer {
    pub image: ThermalImage,
    pub page_image: ThermalImage,
}

impl ImageRenderer {
    pub fn new() -> Self {
        Self {
            image: ThermalImage::new(0),
            page_image: ThermalImage::new(0),
        }
    }

    pub fn render(bytes: &Vec<u8>) -> RenderOutput<ReceiptImage> {
        let mut image_renderer: Box<dyn OutputRenderer<_>> = Box::new(ImageRenderer::new());
        let mut renderer = Renderer::new(&mut image_renderer);
        renderer.render(bytes)
    }
}

pub struct ReceiptImage {
    pub bytes: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

impl OutputRenderer<ReceiptImage> for ImageRenderer {
    fn begin_render(&mut self, context: &mut Context) {
        self.image.empty();
        self.image.set_width(context.graphics.render_area.w);
        self.page_image.set_width(0);
        //Page images should not auto grow in either direction
        //Normally only the width is locked down, but for page mode
        //We want to lock down the height as well
        self.page_image.auto_grow = false;
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

    fn render_graphics(&mut self, context: &mut Context, graphics: &Vec<VectorGraphic>) {
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

    fn render_image(&mut self, context: &mut Context, image: &Image) {
        if context.page_mode.enabled {
            self.page_image.put_render_img(image);
        } else {
            self.image.put_render_img(image);
        }
    }

    fn render_text(
        &mut self,
        context: &mut Context,
        spans: &Vec<TextSpan>,
        x_offset: u32,
        _text_justify: TextJustify,
    ) {
        let canvas = if context.page_mode.enabled {
            &mut self.page_image
        } else {
            &mut self.image
        };

        for span in spans {
            println!("Print text {:?}", span);
            if let Some(_) = &span.dimensions {
                canvas.render_span(x_offset, span);
            }
        }
    }

    fn device_command(&mut self, _context: &mut Context, _command: &DeviceCommand) {}

    fn end_render(&mut self, context: &mut Context) -> ReceiptImage {
        //Add in the left and right margin;
        self.image.expand_to_width(context.graphics.paper_area.w);

        //Feed to the y height to ensure we catch any cut advances
        self.image.expand_to_height(context.graphics.render_area.y);

        let rendered = self.image.copy();

        ReceiptImage {
            width: rendered.0,
            height: rendered.1,
            bytes: rendered.2,
        }
    }
}
