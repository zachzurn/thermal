use std::rc::Rc;
use std::time::Duration;
use thermal_parser::command::DeviceCommand;
use thermal_parser::context::Context;
use crate::image_renderer::thermal_image::{FontFamily, TextLayout, TextSpan, ThermalImage};
use crate::renderer::CommandRenderer;

pub mod thermal_image;

pub struct ImageRenderer{
    pub image: ThermalImage,
    pub text_layout: Option<TextLayout>,
    pub out_path: String,
    out_count: u32
}

impl ImageRenderer {
    pub fn new(out_path: String) -> Self{
        let regular = fontdue::Font::from_bytes(include_bytes!("../../resources/fonts/JetBrainsMonoNL-Medium.ttf") as &[u8], fontdue::FontSettings::default()).unwrap();
        let bold = fontdue::Font::from_bytes(include_bytes!("../../resources/fonts/JetBrainsMonoNL-Bold.ttf") as &[u8], fontdue::FontSettings::default()).unwrap();
        let italic = fontdue::Font::from_bytes(include_bytes!("../../resources/fonts/JetBrainsMonoNL-MediumItalic.ttf") as &[u8], fontdue::FontSettings::default()).unwrap();
        let bold_italic = fontdue::Font::from_bytes(include_bytes!("../../resources/fonts/JetBrainsMonoNL-BoldItalic.ttf") as &[u8], fontdue::FontSettings::default()).unwrap();

        let fonts = Rc::from(FontFamily{ regular, bold, italic, bold_italic });

        Self{
            image: ThermalImage::new(fonts.clone(), 0),
            text_layout: None,
            out_path,
            out_count: 0
        }
    }
}

impl CommandRenderer for ImageRenderer {
    fn begin_render(&mut self, context: &mut Context){
        self.image.set_width(context.available_width_pixels() as usize);
    }

    fn begin_graphics(&mut self, context: &mut Context){
        self.maybe_render_text(context);
    }

    fn draw_rect(&mut self, context: &mut Context, w: usize, h: usize){
        self.image.draw_rect(context.graphics.x, context.graphics.y, w, h);
    }
    fn end_graphics(&mut self, _context: &mut Context){}

    fn draw_image(&mut self, context: &mut Context, bytes: Vec<u8>, width: usize, height: usize){
        self.maybe_render_text(context);
        self.image.put_pixels(context.graphics.x, context.graphics.y, width, height, bytes, false, true);
        if context.text.upside_down { self.image.flip_pixels(context.graphics.x, context.graphics.y, width, height) }
    }

    fn draw_text(&mut self, context: &mut Context, text: String){
        //Here we are avoiding using text layout for single newlines
        //by advancing the newline manually when the text layout is empty
        if self.text_layout.is_none() && text.eq("\n") {
            context.graphics.y += context.text.line_spacing as usize;
            return;
        }

        let span = TextSpan::new(self.image.font.clone(), text.to_string(), context);

        if self.text_layout.is_none() {
            self.text_layout = Some(TextLayout{
                spans: vec![span],
                line_height: context.line_height_pixels() as usize,
                tab_len: context.text.tab_len as usize
            });
        } else {
            if let Some(layout) = &mut self.text_layout {
                layout.spans.push(span);
            }
        }
    }

    fn draw_device_command(&mut self, context: &mut Context, _command: &DeviceCommand){
        self.maybe_render_text(context);
    }

    fn end_render(&mut self, context: &mut Context){
        self.maybe_render_text(context);

        //Simulate post cut feeding
        self.image.add_top_margin(context.line_height_pixels() as usize * 2);

        //Add in the left and right margin;
        self.image.expand_to_width((context.graphics.paper_width * context.graphics.dots_per_inch as f32) as usize);

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
    fn unique_out_path(&mut self) -> String{
        self.out_count = self.out_count.wrapping_add(1);
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or(Duration::new(0,0));
        format!("{}/{}_{}.png", self.out_path.to_string(), now.as_secs() / 60, self.out_count)
    }
    pub fn maybe_render_text(&mut self, context: &mut Context){
        if let Some(layout) = &mut self.text_layout {
            let (_,y) = self.image.draw_text(context.graphics.x as usize, context.graphics.y as usize, self.image.width, layout);
            context.graphics.y = y;
            self.text_layout = None;
        }
    }
}
