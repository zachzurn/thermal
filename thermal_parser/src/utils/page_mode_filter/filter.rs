use std::collections::HashMap;
use lazy_static::lazy_static;
use crate::context::Context;

type PageModeCommands = HashMap<Vec<u8>, &'static str>;

lazy_static! {
    static ref PAGE_MODE_COMMANDS: PageModeCommands = {
        let mut map = HashMap::new();
        map.insert(vec![27, 12], "ESC FF");
        map.insert(vec![27, 76], "ESC L");
        map.insert(vec![27, 84], "ESC T");
        map.insert(vec![27, 87], "ESC W");
        map.insert(vec![29, 36], "GS $");
        map
    };
}

pub fn is_page_mode_activated(context: &Context) -> bool {
    context.is_page_mode
}

pub fn is_page_mode_command(commands: &Vec<u8>) -> bool {
    if let (Some(first), Some(second)) = (commands.get(0), commands.get(1)) {
        PAGE_MODE_COMMANDS.contains_key(&[*first, *second])
    } else {
        false
    }
}
