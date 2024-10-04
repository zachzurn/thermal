use crate::image_renderer::thermal_image::{FontFamily, TextLayout, TextSpan, ThermalImage};
use crate::renderer::CommandRenderer;
use std::mem;
use std::rc::Rc;
use thermal_parser::command::DeviceCommand;
use thermal_parser::context::{Context, PrintDirection, Rotation};

pub mod thermal_image;

pub struct ImageRenderer {
    pub image: ThermalImage,
    pub page_image: ThermalImage,
    pub text_layout: Option<TextLayout>,
    pub out_path: String,
    out_count: u32,
}

impl ImageRenderer {
    pub fn new(out_path: String) -> Self {
        let regular = fontdue::Font::from_bytes(
            include_bytes!("../../resources/fonts/JetBrainsMonoNL-Medium.ttf") as &[u8],
            fontdue::FontSettings::default(),
        )
        .unwrap();
        let bold = fontdue::Font::from_bytes(
            include_bytes!("../../resources/fonts/JetBrainsMonoNL-Bold.ttf") as &[u8],
            fontdue::FontSettings::default(),
        )
        .unwrap();
        let italic = fontdue::Font::from_bytes(
            include_bytes!("../../resources/fonts/JetBrainsMonoNL-MediumItalic.ttf") as &[u8],
            fontdue::FontSettings::default(),
        )
        .unwrap();
        let bold_italic = fontdue::Font::from_bytes(
            include_bytes!("../../resources/fonts/JetBrainsMonoNL-BoldItalic.ttf") as &[u8],
            fontdue::FontSettings::default(),
        )
        .unwrap();

        let fonts = Rc::from(FontFamily {
            regular,
            bold,
            italic,
            bold_italic,
        });

        Self {
            image: ThermalImage::new(fonts.clone(), 0),
            page_image: ThermalImage::new(fonts.clone(), 0),
            text_layout: None,
            out_path,
            out_count: 0,
        }
    }
}

impl CommandRenderer for ImageRenderer {
    fn begin_render(&mut self, context: &mut Context) {
        self.image
            .set_width(context.available_width_pixels() as usize);
        self.page_image.set_width(0);
        //Page images should not auto grow in either direction
        //Normally only the width is locked down, but for page mode
        //We want to lock down the height as well
        self.page_image.auto_grow = false;
    }

    fn begin_page(&mut self, context: &mut Context) {
        self.page_image.set_width(0);
        //TODO make sure we don't print text to a page image that shouldn't be
        self.maybe_render_text(context);
    }

    fn page_area_changed(
        &mut self,
        context: &mut Context,
        rotation: Rotation,
        width: usize,
        height: usize,
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

    fn end_page(&mut self, _context: &mut Context) {}

    fn print_page(&mut self, context: &mut Context) {
        self.maybe_render_text(context);

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
            context.graphics.x,
            context.graphics.y,
            w,
            h,
            pixels,
            false,
            false,
        );

        context.graphics.y += h;
        context.graphics.x = 0;
    }

    fn begin_graphics(&mut self, context: &mut Context) {
        self.maybe_render_text(context);
    }

    fn draw_rect(&mut self, context: &mut Context, w: usize, h: usize) {
        if context.page_mode.enabled {
            self.page_image.draw_rect(
                context.page_mode.render_area.x,
                context.page_mode.render_area.y,
                w,
                h,
            );
        } else {
            self.image
                .draw_rect(context.graphics.x, context.graphics.y, w, h);
        }
    }
    fn end_graphics(&mut self, _context: &mut Context) {}

    fn draw_image(&mut self, context: &mut Context, bytes: Vec<u8>, width: usize, height: usize) {
        self.maybe_render_text(context);

        if context.page_mode.enabled {
            self.page_image.put_pixels(
                context.page_mode.render_area.x,
                context.page_mode.render_area.y,
                width,
                height,
                bytes,
                false,
                true,
            );
        } else {
            self.image.put_pixels(
                context.graphics.x,
                context.graphics.y,
                width,
                height,
                bytes,
                false,
                true,
            );
            if context.text.upside_down {
                self.image
                    .flip_pixels(context.graphics.x, context.graphics.y, width, height)
            }
        }
    }

    fn draw_text(&mut self, context: &mut Context, text: String) {
        //Here we are avoiding using text layout for single newlines
        //by advancing the newline manually when the text layout is empty
        if self.text_layout.is_none() && text.eq("\n") {
            if context.page_mode.enabled {
                context.page_mode.render_area.y += context.text.line_spacing as usize;
            } else {
                context.graphics.y += context.text.line_spacing as usize;
            }
            return;
        }

        let span = TextSpan::new(self.image.font.clone(), text.to_string(), context);

        if self.text_layout.is_none() {
            self.text_layout = Some(TextLayout {
                spans: vec![span],
                line_height: context.line_height_pixels() as usize,
                tab_len: context.text.tab_len as usize,
            });
        } else {
            if let Some(layout) = &mut self.text_layout {
                layout.spans.push(span);
            }
        }
    }

    fn draw_device_command(&mut self, context: &mut Context, _command: &DeviceCommand) {
        self.maybe_render_text(context);
    }

    fn end_render(&mut self, context: &mut Context) {
        self.maybe_render_text(context);

        //Simulate post cut feeding
        self.image
            .add_top_margin(context.line_height_pixels() as usize * 2);

        //Add in the left and right margin;
        self.image.expand_to_width(
            (context.graphics.paper_width * context.graphics.dots_per_inch as f32) as usize,
        );

        //Feed to the y height to ensure we catch any cut advances
        self.image.expand_to_height(context.graphics.y);

        let out_path = self.unique_out_path();

        self.image.save_png(out_path);

        self.image.reset();
        context.graphics.x = 0;
        context.graphics.y = 0;
    }
}

impl ImageRenderer {
    fn page_size(current_dimension: usize, new_offset: usize, new_dimension: usize) -> usize {
        if current_dimension == 0 {
            new_dimension
        } else if new_offset + new_dimension > current_dimension {
            new_offset + new_dimension
        } else {
            current_dimension
        }
    }

    fn unique_out_path(&mut self) -> String {
        self.out_count = self.out_count.wrapping_add(1);
        format!("{}.png", self.out_path.to_string())
    }
    //TODO maybe have the renderer take care of collecting text
    pub fn maybe_render_text(&mut self, context: &mut Context) {
        if let Some(layout) = &mut self.text_layout {
            if context.page_mode.enabled {
                let (_, y) = self.page_image.draw_text(
                    context.page_mode.render_area.x as usize,
                    context.page_mode.render_area.y as usize,
                    context.page_mode.render_area.w,
                    layout,
                );
                context.page_mode.render_area.y = y;
            } else {
                let (_, y) = self.image.draw_text(
                    context.graphics.x as usize,
                    context.graphics.y as usize,
                    self.image.width,
                    layout,
                );
                context.graphics.y = y;
            }

            self.text_layout = None;
        }
    }
}
