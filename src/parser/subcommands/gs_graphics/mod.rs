use crate::parser::Command;
use std::rc::Rc;

pub mod clear_all_download_graphics;
pub mod clear_all_nv_graphics;
pub mod clear_download_graphic;
pub mod clear_nv_graphic;
pub mod define_download_graphics_column;
pub mod define_download_graphics_raster;
pub mod define_nv_graphics_column;
pub mod define_nv_graphics_raster;
pub mod get_download_keycodes;
pub mod get_nv_capacity;
pub mod get_nv_keycodes;
pub mod get_nv_remaining_capacity;
pub mod print_buffer_graphics;
pub mod print_download_graphics;
pub mod print_nv_graphic;
pub mod set_dot_density;
pub mod store_buffer_graphics_raster;
pub mod store_buffer_graphics_table;

pub fn all() -> Rc<Vec<Command>> {
    let all: Vec<Command> = vec![
        clear_all_download_graphics::new(),
        clear_all_nv_graphics::new(),
        clear_download_graphic::new(),
        clear_nv_graphic::new(),
        define_download_graphics_column::new(),
        define_download_graphics_raster::new(),
        define_nv_graphics_column::new(),
        define_nv_graphics_raster::new(),
        define_download_graphics_raster::new(),
        get_download_keycodes::new(),
        get_nv_capacity::new(),
        get_nv_keycodes::new(),
        get_nv_remaining_capacity::new(),
        print_buffer_graphics::new(),
        print_download_graphics::new(),
        print_nv_graphic::new(),
        set_dot_density::new(),
        store_buffer_graphics_raster::new(),
        store_buffer_graphics_table::new(),
    ];

    Rc::new(all)
}