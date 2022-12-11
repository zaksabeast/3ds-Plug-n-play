use super::host_api;
use crate::{
    display::{Screen, TextPrinter},
    memory::GameMemory,
};
use alloc::{string::String, vec::Vec};
use ctr::{error, res::CtrResult};
use wasmi::{Engine, Extern, Func, Linker, Memory, Module, Store, TypedFunc};

pub struct HostState {
    pub(super) text_printer: TextPrinter,
    pub(super) game: GameMemory,
    pub(super) wasm_mem: Option<Memory>,
    pub(super) title_id: u64,
    pub(super) print_buffer: Vec<String>,
}

pub struct Wasm {
    store: Store<HostState>,
    run_frame: Option<TypedFunc<(), ()>>,
}

impl Wasm {
    pub fn new(title_id: u64, game: GameMemory, plugin: &[u8]) -> Result<Self, wasmi::Error> {
        let engine = Engine::default();
        let module = Module::new(&engine, plugin)?;

        let mut store = Store::new(
            &engine,
            HostState {
                title_id,
                game,
                wasm_mem: None,
                text_printer: TextPrinter::default(),
                print_buffer: Vec::with_capacity(30),
            },
        );

        let host_print = Func::wrap(&mut store, host_api::host_print);
        let host_read_mem = Func::wrap(&mut store, host_api::host_read_mem);
        let host_just_pressed = Func::wrap(&mut store, host_api::host_just_pressed);
        let host_is_just_pressed = Func::wrap(&mut store, host_api::host_is_just_pressed);
        let host_write_mem = Func::wrap(&mut store, host_api::host_write_mem);
        let host_reset_print = Func::wrap(&mut store, host_api::host_reset_print);
        let host_set_print_colors = Func::wrap(&mut store, host_api::host_set_print_colors);
        let host_set_print_max_len = Func::wrap(&mut store, host_api::host_set_print_max_len);
        let host_set_print_x = Func::wrap(&mut store, host_api::host_set_print_x);
        let host_set_print_y = Func::wrap(&mut store, host_api::host_set_print_y);
        let host_get_game_title_id = Func::wrap(&mut store, host_api::host_get_game_title_id);

        let mut linker = <Linker<HostState>>::new();
        linker.define("env", "host_print", host_print)?;
        linker.define("env", "host_read_mem", host_read_mem)?;
        linker.define("env", "host_write_mem", host_write_mem)?;
        linker.define("env", "host_just_pressed", host_just_pressed)?;
        linker.define("env", "host_is_just_pressed", host_is_just_pressed)?;
        linker.define("env", "host_reset_print", host_reset_print)?;
        linker.define("env", "host_set_print_colors", host_set_print_colors)?;
        linker.define("env", "host_set_print_max_len", host_set_print_max_len)?;
        linker.define("env", "host_set_print_x", host_set_print_x)?;
        linker.define("env", "host_set_print_y", host_set_print_y)?;
        linker.define("env", "host_get_game_title_id", host_get_game_title_id)?;

        let instance = linker.instantiate(&mut store, &module)?.start(&mut store)?;
        let run_frame = instance
            .get_export(&store, "run_frame")
            .and_then(Extern::into_func)
            .and_then(|func| func.typed::<(), ()>(&mut store).ok());

        Ok(Self { store, run_frame })
    }

    pub fn run_frame(&mut self, screen: &Screen) -> CtrResult {
        if let Some(run_frame) = self.run_frame {
            let state = self.store.state_mut();
            state.print_buffer.clear();

            run_frame
                .call(&mut self.store, ())
                .map_err(|_| error::invalid_value())?;

            let state = self.store.state_mut();
            state
                .text_printer
                .draw_to_screen(screen, &state.print_buffer)?;
        }

        Ok(())
    }
}
