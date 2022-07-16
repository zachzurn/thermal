use crate::parser::command_sets::CommandSet;
use crate::parser::commands::*;
use std::sync::Arc;

//These should always be in alphabetical order
pub fn new() -> CommandSet{
  let commands = vec![
    barcode::new(),
    bitmap::new(),
    cancel_kanji_character_mode::new(),
    cancel::new(),
    carriage_return::new(),
    qr_code::new(),
    default_line_spacing::new(),
    feed_and_cut::new(),
    formfeed::new(),
    graphics::new(),
    horizontal_tab::new(),
    initialize::new(),
    large_bitmap::new(),
    large_graphics::new(),
    linefeed::new(),
    paper_end_sensor::new(),
    print_and_feed_lines::new(),
    print_and_feed::new(),
    print_and_reverse_feed_lines::new(),
    print_stop_sensor::new(),
    pulse::new(),
    request_response_transmission::new(),
    set_absolute_print_pos::new(),
    set_alt_color::new(),
    set_barcode_height::new(),
    set_barcode_width::new(),
    set_black_white_invert::new(),
    set_character_size::new(),
    set_code_table::new(),
    set_double_strike::new(),
    set_emphasis::new(),
    set_font::new(),
    set_graphics_x_y::new(),
    set_hri_print_pos::new(),
    set_international_charset::new(),
    set_justification::new(),
    set_kanji_character_code::new(),
    set_line_spacing::new(),
    set_panel_buttons::new(),
    set_peripheral_device::new(),
    set_print_mode::new(),
    set_relative_vertical_print::new(),
    set_smoothing::new(),
    set_underline::new(),
    set_upside_down::new(),
    transmit_printer_id::new(),
    unknown_data_command::new(),
    unknown_dle::new(),
    unknown_esc_c::new(),
    unknown_esc::new(),
    unknown_fs::new(),
    unknown_gs_graphics::new(),
    unknown_gs::new()
  ];

  CommandSet {
    default: text::new(),
    commands: Arc::from(commands)
  }
}