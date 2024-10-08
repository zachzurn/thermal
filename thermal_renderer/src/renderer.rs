use thermal_parser::command::{Command, CommandType, DeviceCommand};
use thermal_parser::context::{Context, HumanReadableInterface, Rotation};
use thermal_parser::graphics::{
    Barcode, Code2D, GraphicsCommand, Image, Rectangle, TextLayout, TextSpan, VectorGraphic,
};

pub trait CommandRenderer {
    //default implementation
    fn process_command(&mut self, context: &mut Context, command: &Command) {
        match command.kind {
            CommandType::Text => {
                let maybe_text = command.handler.get_text(command, context);
                if let Some(text) = maybe_text {
                    self.collect_text(context, text);
                }
            }
            CommandType::Graphics => {
                self.render_text(context, TextLayout::new(context));

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
                            self.process_code_2d(context, &code_2d);
                        }
                        GraphicsCommand::Barcode(barcode) => {
                            self.process_barcode(context, &barcode);
                        }
                        GraphicsCommand::Image(mut image) => {
                            self.process_image(context, &mut image);
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

                self.process_device_commands(
                    &command.handler.get_device_command(command, context),
                    context,
                );
            }
            CommandType::Control => {
                self.process_device_commands(
                    &command.handler.get_device_command(command, context),
                    context,
                );
            }
            _ => {}
        }
    }

    fn process_device_commands(
        &mut self,
        device_commands: &Option<Vec<DeviceCommand>>,
        context: &mut Context,
    ) {
        if let Some(device_commands) = device_commands {
            for device_command in device_commands {
                self.device_command(context, device_command);

                match device_command {
                    DeviceCommand::BeginPrint => self.begin_render(context),
                    DeviceCommand::EndPrint => {
                        self.render_text(context, TextLayout::new(context));
                        self.end_render(context)
                    }
                    DeviceCommand::FeedLine(num_lines) => {
                        self.render_text(context, TextLayout::new(context));
                        context.newline(*num_lines as u32);
                    }
                    DeviceCommand::Feed(num) => {
                        self.render_text(context, TextLayout::new(context));
                        context.feed(*num as u32);
                    }
                    DeviceCommand::FullCut | DeviceCommand::PartialCut => {
                        self.render_text(context, TextLayout::new(context));
                        context.newline(2);
                    }
                    DeviceCommand::BeginPageMode => {
                        self.render_text(context, TextLayout::new(context));
                        context.page_mode.enabled = true;
                        self.page_begin(context);
                    }
                    DeviceCommand::EndPageMode => {
                        self.page_end(context);
                        context.page_mode.enabled = false
                    }
                    DeviceCommand::PrintPageMode => {
                        self.render_text(context, TextLayout::new(context));
                        self.page_print(context);

                        //Advance the y since a page is being rendered
                        context.graphics.render_area.y += context.page_mode.page_area.h;
                        context.graphics.render_area.x = 0;
                    }
                    DeviceCommand::ChangePageModeDirection => {
                        self.render_text(context, TextLayout::new(context));
                        let (rotation, width, height) = context.page_mode.apply_logical_area();
                        self.page_area_changed(context, rotation, width, height);
                    }
                    _ => {}
                }
            }
        }
    }

    fn process_code_2d(&mut self, context: &mut Context, code_2d: &Code2D) {
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

        self.render_graphics(context, &graphics);
    }

    fn process_barcode(&mut self, context: &mut Context, barcode: &Barcode) {
        println!(
            "Render barcode at X{} Y{}",
            context.get_x(),
            context.get_y()
        );

        let mut graphics = vec![];

        match context.barcode.human_readable {
            HumanReadableInterface::Above | HumanReadableInterface::Both => {
                self.collect_text(context, barcode.text.clone());
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

        self.render_graphics(context, &graphics);

        context.reset_x();
        context.offset_y(barcode.point_height as u32);
        context.offset_y(context.line_height_pixels() as u32);

        match context.barcode.human_readable {
            HumanReadableInterface::Below | HumanReadableInterface::Both => {
                self.collect_text(context, barcode.text.clone());
            }
            _ => {}
        }
    }

    fn process_image(&mut self, context: &mut Context, image: &mut Image) {
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
        self.render_image(context, image);

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

    fn render_graphics(&mut self, context: &mut Context, graphics: &Vec<VectorGraphic>);
    fn render_image(&mut self, context: &mut Context, image: &Image);
    fn text_newline(&mut self, _context: &mut Context) {}
    fn collect_text(&mut self, context: &mut Context, text: TextSpan);
    fn render_text(&mut self, context: &mut Context, layout: TextLayout);

    fn device_command(&mut self, context: &mut Context, command: &DeviceCommand);

    fn end_render(&mut self, context: &mut Context);
}

// TextLayout {
// spans: vec![span],
// line_height: context.line_height_pixels() as u32,
// tab_len: context.text.tab_len as u32,
// }
