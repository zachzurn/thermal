mod pdf417_set_column_count;
mod pdf417_set_row_count;
mod pdf417_set_width;
mod pdf417_set_correction_level;
mod pdf417_set_options;
mod pdf417_store;
mod pdf417_print;
mod pdf417_transmit_size;
mod maxi_transmit_size;
mod qr_transmit_size;
mod gs1_transmit_size;
mod aztec_transmit_size;
mod datamatrix_transmit_size;
mod qr_print;
mod maxi_print;
mod composite_transmit_size;
mod gs1_print;
mod composite_print;
mod aztec_print;
mod datamatrix_print;
mod datamatrix_store;
mod aztec_store;
mod composite_store;
mod gs1_store;
mod maxi_store;
mod qr_store;
mod qr_set_model;
mod qr_set_size;
mod qr_set_correction_level;
mod maxi_set_mode;
mod gs1_set_width;
mod gs1_set_max_width;
mod composite_set_width;
mod composite_set_max_width;
mod composite_set_hri;
mod aztec_set_function_and_layers;
mod aztec_set_size;
mod aztec_set_correction_level;
mod datamatrix_set_options;
mod datamatrix_set_width;

use crate::parser::Command;
use std::rc::Rc;


pub fn all() -> Rc<Vec<Command>> {
  let all: Vec<Command> = vec![
    pdf417_set_column_count::new(),
    pdf417_set_row_count::new(),
    pdf417_set_width::new(),
    pdf417_set_correction_level::new(),
    pdf417_set_options::new(),
    pdf417_store::new(),
    pdf417_print::new(),
    pdf417_transmit_size::new(),
    maxi_transmit_size::new(),
    qr_transmit_size::new(),
    gs1_transmit_size::new(),
    aztec_transmit_size::new(),
    datamatrix_transmit_size::new(),
    qr_print::new(),
    maxi_print::new(),
    composite_transmit_size::new(),
    gs1_print::new(),
    composite_print::new(),
    aztec_print::new(),
    datamatrix_print::new(),
    datamatrix_store::new(),
    aztec_store::new(),
    composite_store::new(),
    gs1_store::new(),
    maxi_store::new(),
    qr_store::new(),
    qr_set_model::new(),
    qr_set_size::new(),
    qr_set_correction_level::new(),
    maxi_set_mode::new(),
    gs1_set_width::new(),
    gs1_set_max_width::new(),
    composite_set_width::new(),
    composite_set_max_width::new(),
    composite_set_hri::new(),
    aztec_set_function_and_layers::new(),
    aztec_set_size::new(),
    aztec_set_correction_level::new(),
    datamatrix_set_options::new(),
    datamatrix_set_width::new()
  ];

  Rc::new(all)
}