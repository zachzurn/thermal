
# Thermal
Thermal Emulator (esc/pos) built in Rust.

This is my first Rust project, so things will be very messy. Feedback and contribution is welcome.

## Goals:
* Cover the whole esc/pos spec besides deprecated commands
* Provide a simple rendering pipeline that makes it easy to render in various formats
* Render to markdown
* Render to an image
* Render to HTML with SVG barcodes and QR Codes
* Allow for the creation of virtual USB and Ethernet printers
	
## Structure:
Commands are the basic building block for parsing ESC/POS code. Each Command defines a list of match characters that a parser can check against.

The parser loops through all Commands looking for matches and then when it finds a match it pushes bytes until the Command's CommandHandler rejects the bytes.

Each Command defines it's own struct that implements the CommandHandler trait which is responsible for receiving bytes and is also responsible for implementing the optional CommandHandler functions.

```rust
pub trait CommandHandler: CloneCommandHandler {
  //Renders text
  fn get_text(&self, _command: &Command, _context: &Context) -> Option<String>{ None }

  //Renders a graphic
  fn get_graphics(&self, _command: &Command, _context: &Context) -> Option<GraphicsCommand> { None }

  //Applies context
  fn apply_context(&self, _command: &Command, _context: &mut Context){}

  //Transmits data back to the client
  fn transmit(&self, _command: &Command, _context: &Context) -> Option<Vec<u8>>{ None }

  //For debugging commands
  fn debug(&self, _command: &Command, _context: &Context) -> String { 
    if _command.data.is_empty() { return format!("{}", _command.name.to_string()) }
    format!("{} {:02X?}", _command.name.to_string(), _command.data) 
  }
  
  //Push data to a command. The command decides what to accept
  fn push(&mut self, _command: &mut Vec<u8>, _byte: u8) -> bool{ 
    return false 
  }
}

```

The parser will return a list of commands that can then be looped to create output. Here is a simple text renderer. When we need to render images, we output them to files.

```rust
let esc_pos = esc_pos::new();
let commands = esc_pos.parse(&bytes);
let mut context = Context::new();

for command in commands {
    
    command.handler.apply_context(&command, &mut context);

    if let Some(gfx) = command.handler.get_graphics(&command, &context){
        match gfx {
            GraphicsCommand::Qrcode(_qr) => todo!(),
            GraphicsCommand::Barcode(_br) => todo!(),
            GraphicsCommand::Image(img) => {
                let filepath = format!("test/gfx{:?}.pbm", context.graphics.graphics_count);
                if let Ok(_) = fs::write(filepath, img.as_pbm()) {}
                context.graphics.graphics_count += 1;
            },
            _ => {}
        }
    }

    if let Some(text) = command.handler.get_text(&command, &context){ print!("{}", text) }

    //Not going to be implemented but if the command wants to transmit data it can implement this
    if let Some(_return_bytes) = command.handler.transmit(&command, &context){};

}
```

The plan is to create a rendering pipeline that makes a predicable set of calls to abstract away the need to loop commands. The pipeline would be a trait that has various rendering methods like:

* Render text
* Render an image
* Render a rectangle
* Render a line

The idea is that if a renderer implements these methods alone, they can render the esc.pos format.

Thanks for listening.

## Fonts included in this repo do not fall under this repos licence

See thermal_render/resources/fonts/OFL.txt for the license. Fonts were obtained from JetBrains Mono repository on Github:
https://github.com/JetBrains/JetBrainsMono

## Inspiration/References:
https://github.com/receipt-print-hq/escpos-tools
https://github.com/local-group/rust-escposify
https://github.com/buntine/barcoders
https://reference.epson-biz.com/modules/ref_escpos/index.php?content_id=72
