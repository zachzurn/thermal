use std::collections::VecDeque;
use std::mem;
use textwrap::WordSeparator;
use thermal_parser::command::{Command, CommandType, DeviceCommand};
use thermal_parser::context::{Context, HumanReadableInterface, Rotation};
use thermal_parser::graphics::{Barcode, Code2D, GraphicsCommand, Image, Rectangle, VectorGraphic};
use thermal_parser::text::{Dimensions, PositionedTextSpan, TextSpan};

pub struct RenderOutput<Output> {
    pub output: Vec<Output>,
    pub errors: Vec<String>,
    pub debug: Vec<String>,
}

pub struct Renderer<'a, Output> {
    renderer: &'a mut Box<dyn OutputRenderer<Output>>,
    output_buffer: Vec<Output>,
    error_buffer: Vec<String>,
    span_buffer: Vec<TextSpan>,
    debug_buffer: Vec<String>,
    context: Context,
}

impl<'a, Output> Renderer<'a, Output> {
    pub fn new(renderer: &'a mut Box<(dyn OutputRenderer<Output> + 'static)>) -> Self {
        Renderer {
            renderer,
            context: Context::new(),
            span_buffer: vec![],
            error_buffer: vec![],
            debug_buffer: vec![],
            output_buffer: vec![],
        }
    }

    pub fn render(&mut self, bytes: &Vec<u8>) -> RenderOutput<Output> {
        let commands = thermal_parser::parse_esc_pos(bytes);

        for command in commands {
            self.process_command(&command);
        }

        let mut output = vec![];
        let mut errors = vec![];
        let mut debug = vec![];

        mem::swap(&mut output, &mut self.output_buffer);
        mem::swap(&mut errors, &mut self.error_buffer);
        mem::swap(&mut debug, &mut self.debug_buffer);

        RenderOutput {
            output,
            errors,
            debug,
        }
    }

    fn debug(&mut self, info: &str) {
        self.debug_buffer.push(info.to_string());
    }

    //default implementation
    fn process_command(&mut self, command: &Command) {
        match command.kind {
            CommandType::Text => {
                let maybe_text = command.handler.get_text(command, &self.context);
                if let Some(text) = maybe_text {
                    self.collect_text(text);
                }
            }
            CommandType::Graphics => {
                self.process_text();

                let maybe_gfx = command.handler.get_graphics(command, &mut self.context);

                if let Some(gfx) = maybe_gfx {
                    match gfx {
                        GraphicsCommand::Error(error) => {
                            self.error_buffer.push(error);
                        }
                        GraphicsCommand::Code2D(code_2d) => {
                            self.process_code_2d(&code_2d);
                        }
                        GraphicsCommand::Barcode(barcode) => {
                            self.process_barcode(&barcode);
                        }
                        GraphicsCommand::Image(mut image) => {
                            self.process_image(&mut image);
                        }
                        GraphicsCommand::Rectangle(_) => {}
                        GraphicsCommand::Line(_) => {}
                    }
                }
            }
            CommandType::Context => {
                command.handler.apply_context(command, &mut self.context);
            }
            CommandType::ContextControl => {
                command.handler.apply_context(command, &mut self.context);

                let device_commands = &command
                    .handler
                    .get_device_command(command, &mut self.context);
                self.process_device_commands(device_commands);
            }
            CommandType::Control => {
                let device_commands = &command
                    .handler
                    .get_device_command(command, &mut self.context);
                self.process_device_commands(device_commands);
            }
            _ => {}
        }
    }

    fn process_device_commands(&mut self, device_commands: &Option<Vec<DeviceCommand>>) {
        if let Some(device_commands) = device_commands {
            for device_command in device_commands {
                self.renderer
                    .device_command(&mut self.context, device_command);

                match device_command {
                    DeviceCommand::BeginPrint => self.renderer.begin_render(&mut self.context),
                    DeviceCommand::EndPrint => {
                        self.process_text();
                        let output = self.renderer.end_render(&mut self.context);
                        self.output_buffer.push(output);
                    }
                    DeviceCommand::FeedLine(num_lines) => {
                        self.process_text();
                        self.context.newline(*num_lines as u32);
                    }
                    DeviceCommand::Feed(num) => {
                        self.process_text();
                        self.context.feed(*num as u32);
                    }
                    DeviceCommand::FullCut | DeviceCommand::PartialCut => {
                        self.process_text();
                        self.context.newline(2);
                    }
                    DeviceCommand::BeginPageMode => {
                        self.process_text();
                        self.context.page_mode.enabled = true;
                        self.renderer.page_begin(&mut self.context);
                    }
                    DeviceCommand::EndPageMode => {
                        self.renderer.page_end(&mut self.context);
                        self.context.page_mode.enabled = false
                    }
                    DeviceCommand::PrintPageMode => {
                        self.process_text();
                        self.renderer.render_page(&mut self.context);

                        //Advance the y since a page is being rendered
                        self.context.graphics.render_area.y += self.context.page_mode.page_area.h;
                        self.context.graphics.render_area.x = 0;
                    }
                    DeviceCommand::ChangePageModeDirection => {
                        self.process_text();
                        let (rotation, width, height) = self.context.page_mode.apply_logical_area();
                        self.renderer
                            .page_area_changed(&mut self.context, rotation, width, height);
                    }
                    _ => {}
                }
            }
        }
    }

    fn process_code_2d(&mut self, code_2d: &Code2D) {
        let context = &mut self.context;
        let mut graphics = vec![];

        let mut i = 1;
        let origin_x =
            context.calculate_justification(code_2d.points.len() as u32 * code_2d.point_width);

        for p in &code_2d.points {
            if i != 1 && i % code_2d.width == 1 {
                context.set_x(origin_x);
                context.offset_y(code_2d.point_height as u32);
            }

            if *p > 0 {
                graphics.push(VectorGraphic::Rectangle(Rectangle {
                    x: context.get_x(),
                    y: context.get_y(),
                    w: code_2d.point_width as u32,
                    h: code_2d.point_height as u32,
                }));
            }
            context.offset_x(code_2d.point_width as u32);
            i += 1;
        }

        context.reset_x();

        self.renderer.render_graphics(context, &graphics);
    }

    fn process_barcode(&mut self, barcode: &Barcode) {
        let mut graphics = vec![];

        match self.context.barcode.human_readable {
            HumanReadableInterface::Above | HumanReadableInterface::Both => {
                self.collect_text(barcode.text.clone());
                self.process_text();
            }
            _ => {}
        }

        self.context.set_x(
            self.context
                .calculate_justification(barcode.points.len() as u32 * barcode.point_width as u32),
        );

        for bp in &barcode.points {
            if *bp > 0 {
                graphics.push(VectorGraphic::Rectangle(Rectangle {
                    x: self.context.get_x(),
                    y: self.context.get_y(),
                    w: barcode.point_width as u32,
                    h: barcode.point_height as u32,
                }));
            }
            self.context.offset_x(barcode.point_width as u32);
        }

        self.renderer.render_graphics(&mut self.context, &graphics);

        self.context.reset_x();
        self.context.offset_y(barcode.point_height as u32);
        self.context
            .offset_y(self.context.line_height_pixels() as u32);

        match self.context.barcode.human_readable {
            HumanReadableInterface::Below | HumanReadableInterface::Both => {
                self.collect_text(barcode.text.clone());
                self.process_text();
            }
            _ => {}
        }
    }

    fn process_image(&mut self, image: &mut Image) {
        let context = &mut self.context;

        if image.advances_y && context.get_x() == 0 {
            context.set_x(context.calculate_justification(image.w));
        }

        //Images that exceed the render width will be bumped down to the next line
        if !image.advances_y
            && image.w + context.get_x() > context.get_base_x() + context.get_width()
        {
            context.newline(1);
        }

        image.x = context.get_x();
        image.y = context.get_y();
        self.renderer.render_image(context, image);

        //Start a new line after the image
        if image.advances_y {
            context.reset_x();
            context.offset_y(image.h);
            context.offset_y(context.line_height_pixels());
        } else {
            context.offset_x(image.w);
        }
    }

    fn collect_text(&mut self, text: TextSpan) {
        self.span_buffer.push(text);
    }

    //Returns the next line that can be created from the list of words.
    //Also modifies the words vec to have leftover words
    fn extract_line_from_words(&mut self, words: &mut VecDeque<TextSpan>) -> Option<Vec<TextSpan>> {
        let mut line: Vec<TextSpan> = vec![];

        while let Some(mut word) = words.pop_front() {
            //Calculate available width every loop
            let avail_width = self.context.get_available_width();
            let word_width = word.text.len() as u32 * word.character_width;

            if word_width <= avail_width {
                //Word fits into the line
                word.get_dimensions(&self.context);
                self.context.offset_x(word.get_width());
                line.push(word);
                continue;
            } else if word_width > avail_width {
                //Break the word into parts for super long words
                let broken = word.break_apart((avail_width / word.character_width) as usize);
                word.get_dimensions(&self.context);
                self.context.offset_x(word.get_width());
                line.push(word);
                words.push_front(broken);
                //Line is full
                return Some(line);
            } else {
                //Add word to a newline
            }
        }

        None
    }

    fn process_text(&mut self) {
        if self.span_buffer.is_empty() {
            return;
        }

        let mut words: Vec<TextSpan> = vec![];

        for span in &self.span_buffer {
            if span.text.eq("\n") {
                self.context.newline(1);
                continue;
            }

            words.append(&mut span.break_into_words().into());
        }

        self.span_buffer.clear();

        let mut lines: Vec<Vec<TextSpan>> = vec![];
        let mut current_line: Vec<TextSpan> = vec![];
        words.reverse();

        while let Some(mut word) = words.pop() {
            //Calculate available width every loop
            let avail_width = self.context.get_available_width();
            let word_width = word.text.len() as u32 * word.character_width;

            if word_width <= avail_width {
                //Word fits into the line, add it
                word.get_dimensions(&self.context);
                self.context.offset_x(word.get_width());
                current_line.push(word);
                continue;
            } else if word_width > avail_width {
                //Break the word into parts for super long words
                let broken = word.break_apart(
                    (avail_width / word.character_width) as usize,
                    (self.context.get_width() / word.character_width) as usize,
                );

                //Add the part that fits into the line
                word.get_dimensions(&self.context);
                self.context.offset_x(word.get_width());
                current_line.push(word);

                //Line is full
                let mut finished_line = vec![];
                mem::swap(&mut current_line, &mut finished_line);
                lines.push(finished_line);

                //Fill new lines with words
                for broke in broken {
                    //TODO
                }
            } else {
                //Add word to a newline
                let mut finished_line = vec![];
                mem::swap(&mut current_line, &mut finished_line);
                lines.push(finished_line);
                self.context.newline(1);
                word.get_dimensions(&self.context);
                current_line.push(word);
            }
        }
    }
}

pub trait OutputRenderer<Output> {
    fn begin_render(&mut self, context: &mut Context);
    fn page_begin(&mut self, _context: &mut Context);
    fn page_area_changed(
        &mut self,
        _context: &mut Context,
        _rotation: Rotation,
        _width: u32,
        _height: u32,
    );
    fn page_end(&mut self, _context: &mut Context);
    fn render_page(&mut self, _context: &mut Context);
    fn render_graphics(&mut self, context: &mut Context, graphics: &Vec<VectorGraphic>);
    fn render_image(&mut self, context: &mut Context, image: &Image);
    fn render_text(&mut self, context: &mut Context, lines: &Vec<TextSpan>);
    fn device_command(&mut self, context: &mut Context, command: &DeviceCommand);
    fn end_render(&mut self, context: &mut Context) -> Output;
}
