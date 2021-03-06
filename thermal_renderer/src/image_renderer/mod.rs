extern crate nuid;

use std::rc::Rc;
use thermal_parser::command::DeviceCommand;
use thermal_parser::context;
use thermal_parser::context::{Context, TextJustify, TextUnderline};
use crate::image_renderer::thermal_image::{FontFamily, TextLayout, TextSpan, ThermalImage};
use crate::renderer::CommandRenderer;

pub mod thermal_image;

pub struct ImageRenderer{
    pub image: ThermalImage,
    pub text_layout: Option<TextLayout>,
    pub out_path: String
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
            out_path
        }
    }
}

impl CommandRenderer for ImageRenderer {
    fn begin_render(&mut self, context: &mut Context){
        println!("BEGIN RENDER {}", context.available_width_pixels());
        self.image.set_width(context.available_width_pixels() as usize);
    }

    fn begin_graphics(&mut self, context: &mut Context){
        self.maybe_render_text(context);
    }

    fn draw_rect(&mut self, context: &mut Context, w: usize, h: usize){
        self.image.draw_rect(context.graphics.x as usize, context.graphics.y as usize, w, h);
    }
    fn end_graphics(&mut self, context: &mut Context){}

    fn draw_image(&mut self, context: &mut Context, bytes: Vec<u8>, width: usize, height: usize){
        self.maybe_render_text(context);
        self.image.put_pixels(context.graphics.x, context.graphics.y, width, height, bytes, false);
    }

    fn draw_text(&mut self, context: &mut Context, text: String){
        let mut span = TextSpan::new(self.image.font.clone(), text.to_string(), context);

        if self.text_layout.is_none() {
            self.text_layout = Some(TextLayout{
                spans: vec![span],
                line_height: context.line_height_pixels() as usize,
                justify: context.text.justify.clone()
            });
        } else {
            if let Some(layout) = &mut self.text_layout {
                layout.spans.push(span);
            }
        }
    }

    fn draw_device_command(&mut self, context: &mut Context, command: &DeviceCommand){
        self.maybe_render_text(context);
    }

    fn end_render(&mut self, context: &mut Context){
        println!("END RENDER");

        self.maybe_render_text(context);

        self.image.save_png(format!("{}/{}.png", self.out_path.to_string(), nuid::next()));

        self.image.reset();
        context.graphics.x = 0;
        context.graphics.y = 0;
    }

}

impl ImageRenderer {
    pub fn maybe_render_text(&mut self, context: &mut Context){
        if let Some(layout) = &mut self.text_layout {
            let (_,y) = self.image.draw_text(context.graphics.x as usize, context.graphics.y as usize, self.image.width, layout);
            context.graphics.y = y;
            self.text_layout = None;
        }
    }
}
