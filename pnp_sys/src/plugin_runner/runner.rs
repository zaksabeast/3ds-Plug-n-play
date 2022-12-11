use super::{loader::PluginLoaderMenu, wasm::Wasm};
use crate::{display::Screen, memory::GameMemory};
use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};
use ctr::{error, fs, hid, hid::InterfaceDevice, res::CtrResult};

pub struct PluginRunner {
    title_id: u64,
    wasm: Wasm,
    plugin_menu: PluginLoaderMenu,
    game: GameMemory,
    show_plugin_menu: bool,
}

impl PluginRunner {
    fn read_plugins_in_directory(path: &str) -> Option<Vec<String>> {
        let result = fs::read_dir(path)
            .ok()?
            .filter_map(|dir_entry| {
                let dir_entry_name = dir_entry.name();
                if dir_entry_name.ends_with(".wasm") {
                    Some(format!("{}/{}", path, dir_entry_name))
                } else {
                    None
                }
            })
            .collect();
        Some(result)
    }

    fn read_plugins_for_title(title_id: u64) -> Vec<String> {
        let paths = ["sd:/pnp".to_string(), format!("sd:/pnp/{:016X}", title_id)];

        paths
            .iter()
            .filter_map(|path| Self::read_plugins_in_directory(path))
            .flatten()
            .collect()
    }

    pub fn new(title_id: u64, game: GameMemory) -> Option<Self> {
        let plugins = Self::read_plugins_for_title(title_id);
        let plugin_path = plugins.first()?;
        let plugin = fs::read(plugin_path).ok()?;
        Some(Self {
            title_id,
            game,
            wasm: Wasm::new(title_id, game, &plugin).ok()?,
            plugin_menu: PluginLoaderMenu::new(plugins),
            show_plugin_menu: false,
        })
    }

    pub fn run_frame(&mut self, screen: Screen) -> CtrResult {
        hid::Global::scan_input();

        // Only large_mem can switch plugins
        #[cfg(feature = "large_mem")]
        if hid::Global::is_just_pressed(hid::Button::Start | hid::Button::Ddown) {
            self.show_plugin_menu = !self.show_plugin_menu;
        }

        if self.show_plugin_menu {
            self.plugin_menu.run_frame(&screen)?;

            if hid::Global::is_just_pressed(hid::Button::A) {
                let plugin_path = self.plugin_menu.value();
                let plugin = fs::read(plugin_path)?;
                self.show_plugin_menu = false;
                self.wasm = Wasm::new(self.title_id, self.game, &plugin)
                    .map_err(|_| error::invalid_value())?;
            }
        } else {
            self.wasm
                .run_frame(&screen)
                .map_err(|_| error::invalid_result_value())?;
        }

        screen.flush()?;

        Ok(())
    }
}
