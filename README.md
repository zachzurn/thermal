# Thermal
Thermal Emulator (esc/pos) built in Rust.

This is a work in progress and should not in any way be used right now.

This is my first Rust project, so things will be very messy. Feedback and contribution is welcome.

Goal is to have a functional esc/pos emulator that has a state that is written to a file for onboard images and whatnot that can render to.

* text file
* json debug file
* image file
* html file


Currently the initial parser framework has been set up and I am working on implementing the rest of the basic commands.

Commands can be implemented easily by creating a command in the commands module (parser/commands) and then including the command in the esc_pos command_set commands Vec.

```rust
use std::sync::Arc;

use crate::parser::*;

struct Handler;

impl CommandHandler for Handler {
  //implementation for this command, different commands can override the expected functions (see CommandHandler trait)
  fn get_text(&self, _command: &Command) -> Option<String>{ 
    Some("\r".to_string())
  }
}

pub fn command() -> Command {
  Command::new(
    "Line Feed", //Name of the command
    vec![CR], //List of u8's to match
    CommandType::Text, //General category of the command, useful for rendering
    DataType::Empty, //Data type for the command, useful for parsing
    Arc::new(Handler{}) //implementation for the command (See CommandHandler struct)
  )
}
```

Commands have two type identifiers:

CommandType

```rust
pub enum CommandType {
  Control, //Commands that are purely control commands like initialize printer
  Text, //Commands that display text
  TextContext, //Commands that mutate the text rendering context
  Image, //Commands that display images
  Graphics, //Commands that draw
  GraphicsContext, //Commands that mutate the graphics context
  Unknown //Unknown commands (see Unknown in DataType below)
}
```

DataType tells the parser how to parse the command. Some commands have Single arguments, some commands need custom parsing.

```rust
pub enum DataType {
  Empty, //Command has no arguments
  Single, //Command has a single argument
  Double, //Command has two arguments
  Triple, //Command has three arguments
  Text, //Command should take bytes until a new command is matched
  Bitmap, //Command should parse a bitmaps metadata and pull in the proper amount of bytes
  Unknown //Command should take bytes until a new command is matched. This helps us 
          //learn unimplemented commands by specifying a single top level control command
}
```
