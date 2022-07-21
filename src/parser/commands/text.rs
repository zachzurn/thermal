use std::str::from_utf8;

use crate::parser::*;

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {
    fn get_text(&self, command: &Command, _context: &Context) -> Option<String> {
        //TODO we may need to add the codepage to the context and do proper conversion instead of just using utf8
        match from_utf8(&command.data as &[u8]) {
            Ok(str) => return Some(str.to_string()),
            Err(err) => {
                print!("UTF8 TEXT ERROR {} {:02X?}", err, &command.data);
                return None;
            }
        };
    }
    fn debug(&self, command: &Command, context: &Context) -> String {
        self.get_text(command, context).unwrap_or("".to_string())
    }
}

pub fn new() -> Command {
    Command::new(
      "Text",
      vec![],
      CommandType::Text,
      DataType::Text,
      Box::new(Handler {}),
    )
}