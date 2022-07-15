use crate::parser::command_sets::CommandSet;
use crate::parser::commands::*;
use std::sync::Arc;


// pub mod initialize;
// pub mod cancel;

// pub mod text;
// pub mod horizontal_tab;
// pub mod linefeed;
// pub mod carriage_return;
// pub mod set_justification;

// pub mod default_line_spacing;
// pub mod enable_upside_down;
// pub mod set_line_spacing;

// pub mod bit_image;

// pub mod unknown_dle;
// pub mod unknown_esc;
// pub mod unknown_fs;
// pub mod unknown_gs;

pub fn get() -> CommandSet{
  let commands = vec![
    bitmap::command(),
    cancel::command(),
    carriage_return::command(),
    default_line_spacing::command(),
    formfeed::command(),
    horizontal_tab::command(),
    initialize::command(),
    linefeed::command(),
    paper_end_sensor::command(),
    print_and_feed_lines::command(),
    print_and_feed::command(),
    print_and_reverse_feed_lines::command(),
    print_stop_sensor::command(),
    pulse::command(),
    set_absolute_print_pos::command(),
    set_alt_color::command(),
    set_code_table::command(),
    set_double_strike::command(),
    set_emphasis::command(),
    set_font::command(),
    set_international_charset::command(),
    set_justification::command(),
    set_line_spacing::command(),
    set_panel_buttons::command(),
    set_peripheral_device::command(),
    set_print_mode::command(),
    set_underline::command(),
    set_upside_down::command(),
    unknown_a::command(),
    unknown_b::command(),
    unknown_dle::command(),
    unknown_esc::command(),
    unknown_fs::command(),
    unknown_gs::command()
  ];

  CommandSet {
    default: text::command(),
    commands: Arc::from(commands)
  }
}