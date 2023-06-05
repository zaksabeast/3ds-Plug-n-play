use super::menu::Menu;
use crate::display::{Screen, TextPrinter};
use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};
use ctr::{hid, hid::InterfaceDevice, res::CtrResult};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const GIT_HASH: &str = env!("GIT_HASH");

pub struct PluginLoaderMenu {
    print_settings: TextPrinter,
    print_buffer: Vec<String>,
    menu: Menu,
}

impl PluginLoaderMenu {
    pub fn new(plugins: Vec<String>) -> Self {
        Self {
            print_settings: TextPrinter::new_with_max_len(47),
            print_buffer: Vec::with_capacity(30),
            menu: Menu::new(plugins),
        }
    }

    pub fn value(&self) -> &str {
        self.menu.value()
    }

    pub fn run_frame(&mut self, screen: &Screen) -> CtrResult {
        self.print_buffer.clear();

        if hid::Global::is_just_pressed(hid::Button::Ddown) {
            self.menu.cursor_down()
        } else if hid::Global::is_just_pressed(hid::Button::Dup) {
            self.menu.cursor_up()
        }

        self.print_buffer
            .push(format!("Plugin Menu {} {}", VERSION, GIT_HASH));
        self.print_buffer.push("".to_string());
        self.menu.push_menu_to_buffer(&mut self.print_buffer);

        self.print_settings
            .draw_to_screen(screen, &self.print_buffer)
    }
}
