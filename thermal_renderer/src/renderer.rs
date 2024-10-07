use thermal_parser::command::{Command, CommandType, DeviceCommand};
use thermal_parser::context::{Context, HumanReadableInterface, Rotation};
use thermal_parser::graphics::{
    Barcode, Code2D, GraphicsCommand, Image, Rectangle, TextLayout, TextSpan, VectorGraphic,
};

pub trait CommandRenderer {
    //default implementation
    fn process_command(&mut self, context: &mut Context, command: &Command) {
        if command.kind != CommandType::Text {
            //Before any non text command, we call this to notify
            // a possible end of a continuous list of text spans
            self.text_span_collect(context, TextLayout::new(context));
        }

        match command.kind {
            CommandType::Text => {
                let maybe_text = command.handler.get_text(command, context);
                if let Some(text) = maybe_text {
                    if text.text.eq("\n") {
                        context.newline(1);
                        self.text_newline(context);
                    } else {
                        self.text_span(context, text);
                    }
                }
            }
            CommandType::Graphics => {
                let maybe_gfx = command.handler.get_graphics(command, context);

                if let Some(gfx) = maybe_gfx {
                    match gfx {
                        GraphicsCommand::Error(error) => {
                            eprintln!(
                                "[{}] Graphics Error Encountered: {}",
                                command.handler.debug(command, context),
                                error
                            );
                        }
                        GraphicsCommand::Code2D(code_2d) => {
                            self.render_code_2d(context, &code_2d);
                        }
                        GraphicsCommand::Barcode(barcode) => {
                            self.render_barcode(context, &barcode);
                        }
                        GraphicsCommand::Image(mut image) => {
                            self.render_image(context, &mut image);
                        }
                        GraphicsCommand::Rectangle(_) => {}
                        GraphicsCommand::Line(_) => {}
                    }
                }
            }
            CommandType::Context => {
                command.handler.apply_context(command, context);
            }
            CommandType::ContextControl => {
                command.handler.apply_context(command, context);

                self.handle_device_commands(
                    &command.handler.get_device_command(command, context),
                    context,
                );
            }
            CommandType::Control => {
                self.handle_device_commands(
                    &command.handler.get_device_command(command, context),
                    context,
                );
            }
            _ => {}
        }
    }

    fn handle_device_commands(
        &mut self,
        device_commands: &Option<Vec<DeviceCommand>>,
        context: &mut Context,
    ) {
        if let Some(device_commands) = device_commands {
            for device_command in device_commands {
                self.device_command(context, device_command);

                match device_command {
                    DeviceCommand::BeginPrint => self.begin_render(context),
                    DeviceCommand::EndPrint => self.end_render(context),
                    DeviceCommand::FeedLine(num_lines) => {
                        let advance = context.line_height_pixels() * *num_lines as u32;

                        if context.page_mode.enabled {
                            context.page_mode.render_area.y += advance;
                        } else {
                            context.graphics.render_area.y += advance;
                        }
                    }
                    DeviceCommand::Feed(num) => {
                        let advance = context.motion_unit_y_pixels() * *num as u32;

                        if context.page_mode.enabled {
                            context.page_mode.render_area.y += advance;
                        } else {
                            context.graphics.render_area.y += advance;
                        }
                    }
                    DeviceCommand::FullCut | DeviceCommand::PartialCut => {
                        context.graphics.render_area.y += context.line_height_pixels() * 2;
                    }
                    DeviceCommand::BeginPageMode => {
                        context.page_mode.enabled = true;
                        self.page_begin(context);
                    }
                    DeviceCommand::EndPageMode => {
                        self.page_end(context);
                        context.page_mode.enabled = false
                    }
                    DeviceCommand::PrintPageMode => {
                        self.page_print(context);

                        //Advance the y since a page is being rendered
                        context.graphics.render_area.y += context.page_mode.page_area.h;
                        context.graphics.render_area.x = 0;
                    }
                    DeviceCommand::ChangePageModeDirection => {
                        let (rotation, width, height) = context.page_mode.apply_logical_area();
                        self.page_area_changed(context, rotation, width, height);
                    }
                    _ => {}
                }
            }
        }
    }

    fn render_code_2d(&mut self, context: &mut Context, code_2d: &Code2D) {
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

        self.graphics(context, &graphics);
    }

    fn render_barcode(&mut self, context: &mut Context, barcode: &Barcode) {
        let mut graphics = vec![];

        match context.barcode.human_readable {
            HumanReadableInterface::Above | HumanReadableInterface::Both => {
                self.text_span(context, barcode.text.clone());
            }
            _ => {}
        }

        context.set_x(
            context
                .calculate_justification(barcode.points.len() as u32 * barcode.point_width as u32),
        );

        for bp in &barcode.points {
            if *bp > 0 {
                graphics.push(VectorGraphic::Rectangle(Rectangle {
                    x: context.get_x(),
                    y: context.get_y(),
                    w: barcode.point_width as u32,
                    h: barcode.point_height as u32,
                }));
            }
            context.offset_x(barcode.point_width as u32);
        }

        context.reset_x();
        context.offset_y(barcode.point_height as u32);
        context.offset_y(context.line_height_pixels() as u32);

        match context.barcode.human_readable {
            HumanReadableInterface::Below | HumanReadableInterface::Both => {
                self.text_span(context, barcode.text.clone());
            }
            _ => {}
        }
    }

    fn render_image(&mut self, context: &mut Context, image: &mut Image) {
        if image.advances_y && context.get_x() == 0 {
            context.set_x(context.calculate_justification(image.w));
        }

        //Images that exceed the render width will be bumped down to the next line
        if !image.advances_y
            && image.w + context.get_x() > context.get_base_x() + context.get_width()
        {
            context.reset_x();
            context.offset_y(context.line_height_pixels());
        }

        image.x = context.get_x();
        image.y = context.get_y();
        self.image(context, image);

        //Start a new line after the image
        if image.advances_y {
            context.reset_x();
            context.offset_y(image.h);
            context.offset_y(context.line_height_pixels());
        } else {
            context.offset_x(image.w);
        }
    }

    fn begin_render(&mut self, context: &mut Context);

    //Page mode is optional for implementing renderers
    //and is off by default. No page mode rendering commands
    //will come through
    fn page_mode_supported() -> bool;
    fn page_begin(&mut self, _context: &mut Context) {}
    fn page_area_changed(
        &mut self,
        _context: &mut Context,
        _rotation: Rotation,
        _width: u32,
        _height: u32,
    ) {
    }
    fn page_end(&mut self, _context: &mut Context) {}
    fn page_print(&mut self, _context: &mut Context) {}

    fn graphics(&mut self, context: &mut Context, graphics: &Vec<VectorGraphic>);
    fn image(&mut self, context: &mut Context, image: &Image);
    fn text_newline(&mut self, _context: &mut Context) {}
    fn text_span(&mut self, context: &mut Context, text: TextSpan);
    fn text_span_collect(&mut self, context: &mut Context, layout: TextLayout);

    fn device_command(&mut self, context: &mut Context, command: &DeviceCommand);

    fn end_render(&mut self, context: &mut Context);
}

// TextLayout {
// spans: vec![span],
// line_height: context.line_height_pixels() as u32,
// tab_len: context.text.tab_len as u32,
// }
