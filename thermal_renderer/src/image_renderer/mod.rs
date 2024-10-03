use crate::image_renderer::thermal_image::{FontFamily, TextLayout, TextSpan, ThermalImage};
use crate::renderer::CommandRenderer;
use std::mem;
use std::rc::Rc;
use thermal_parser::command::DeviceCommand;
use thermal_parser::context::{Context, PrintDirection};

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
        self.page_image.set_width(context.page_mode.w);
        self.page_image.ensure_height(context.page_mode.h);
        self.maybe_render_text(context);
    }

    fn page_area_changed(&mut self, context: &mut Context) {
        let current_width = self.page_image.width;
        let current_height = self.page_image.get_height();
        let current_empty = current_width == 0 || current_height == 0;

        let mut new_width = context.page_mode.logical_w;
        let mut new_height = context.page_mode.logical_h;

        //Rotated directions need swapped dimensions
        match context.page_mode.dir {
            PrintDirection::TopRight2Bottom | PrintDirection::BottomLeft2Top => {
                mem::swap(&mut new_width, &mut new_height);
            }
            _ => {}
        }

        //No need to make any adjustments for smaller page area
        //For bigger area, we reset the image and put the old image in place
        if current_width < new_width || current_height < new_height || current_empty {
            let copy = self.page_image.copy();
            self.page_image.set_width(new_width);
            self.page_image.ensure_height(new_height);
            self.page_image
                .put_pixels(0, 0, copy.0, copy.1, copy.2, false, false);
        }
    }

    fn page_direction_changed(&mut self, context: &mut Context) {
        self.page_image.set_print_direction(&context.page_mode.dir);
    }

    fn end_page(&mut self, _context: &mut Context) {}

    fn print_page(&mut self, context: &mut Context) {
        self.maybe_render_text(context);

        let (w, h, pixels) = self.page_image.copy();

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
            self.page_image
                .draw_rect(context.page_mode.x, context.page_mode.y, w, h);
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
                context.page_mode.x,
                context.page_mode.y,
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
                context.page_mode.y += context.text.line_spacing as usize;
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
        self.image.ensure_height(context.graphics.y);

        let out_path = self.unique_out_path();

        self.image.save_png(out_path);

        self.image.reset();
        context.graphics.x = 0;
        context.graphics.y = 0;
    }
}

impl ImageRenderer {
    fn unique_out_path(&mut self) -> String {
        self.out_count = self.out_count.wrapping_add(1);
        format!("{}.png", self.out_path.to_string())
    }
    pub fn maybe_render_text(&mut self, context: &mut Context) {
        if let Some(layout) = &mut self.text_layout {
            if context.page_mode.enabled {
                let (_, y) = self.page_image.draw_text(
                    context.page_mode.x as usize,
                    context.page_mode.y as usize,
                    self.page_image.width,
                    layout,
                );
                context.page_mode.y = y;
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
