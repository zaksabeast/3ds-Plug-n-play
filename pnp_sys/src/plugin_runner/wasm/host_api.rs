use super::instance::HostState;
use crate::display::{Color, TextPrinter};
use alloc::{string::String, vec};
use ctr::{hid, hid::InterfaceDevice};
use wasmi::{Caller, Memory};

#[inline(always)]
fn get_caller_memory(caller: &mut Caller<'_, HostState>) -> Memory {
    let missing_mem = caller.host_data().wasm_mem.is_none();
    if missing_mem {
        let memory = caller
            .get_export("memory")
            .expect("`memory` export not found")
            .into_memory()
            .expect("export name `memory` is not of memory type");
        caller.host_data_mut().wasm_mem = Some(memory);
    }

    caller.host_data().wasm_mem.unwrap()
}

#[inline(always)]
pub fn host_set_print_colors(
    mut caller: Caller<'_, HostState>,
    text_color: u32,
    background_color: u32,
) {
    let host_data = caller.host_data_mut();
    host_data
        .text_printer
        .set_text_color(Color::new(text_color));
    host_data
        .text_printer
        .set_background_color(Color::new(background_color));
}

#[inline(always)]
pub fn host_set_print_max_len(mut caller: Caller<'_, HostState>, max_len: u32) {
    let host_data = caller.host_data_mut();
    host_data.text_printer.set_max_len(max_len as u8);
}

#[inline(always)]
pub fn host_set_print_x(mut caller: Caller<'_, HostState>, x: u32) {
    let host_data = caller.host_data_mut();
    host_data.text_printer.set_print_x(x);
}

#[inline(always)]
pub fn host_set_print_y(mut caller: Caller<'_, HostState>, y: u32) {
    let host_data = caller.host_data_mut();
    host_data.text_printer.set_print_y(y);
}

#[inline(always)]
pub fn host_reset_print(mut caller: Caller<'_, HostState>) {
    let host_data = caller.host_data_mut();
    host_data.text_printer = TextPrinter::default();
}

#[inline(always)]
pub fn host_print(mut caller: Caller<'_, HostState>, ptr: u32, size: u32) {
    let mut string = vec![0; size as usize];
    let result = get_caller_memory(&mut caller).read(&caller, ptr as usize, &mut string);

    if result.is_err() {
        return;
    }

    if let Ok(string) = String::from_utf8(string) {
        let host_data = caller.host_data_mut();
        host_data.print_buffer.push(string);
    }
}

#[inline(always)]
pub fn host_read_mem(
    mut caller: Caller<'_, HostState>,
    game_address: u32,
    size: u32,
    out_ptr: u32,
) {
    if let Some(buf) = caller.host_data().game.read(game_address, size as usize) {
        let memory = get_caller_memory(&mut caller);

        // There's not really much we can do if this fails.
        #[allow(unused_must_use)]
        memory.write(caller, out_ptr as usize, buf);
    }
}

#[inline(always)]
pub fn host_write_mem(
    mut caller: Caller<'_, HostState>,
    game_address: u32,
    size: u32,
    in_ptr: u32,
) {
    if let Some(buf) = caller
        .host_data()
        .game
        .write_buf(game_address, size as usize)
    {
        let memory = get_caller_memory(&mut caller);

        // There's not really much we can do if this fails.
        #[allow(unused_must_use)]
        memory.read(caller, in_ptr as usize, buf);
    }
}

#[inline(always)]
pub fn host_just_pressed(mut _caller: Caller<'_, HostState>) -> u32 {
    hid::Global::just_down_buttons().into()
}

#[inline(always)]
pub fn host_is_just_pressed(mut _caller: Caller<'_, HostState>, io_bits: u32) -> u32 {
    hid::Global::is_just_pressed(io_bits) as u32
}

#[inline(always)]
pub fn host_get_game_title_id(caller: Caller<'_, HostState>) -> u64 {
    caller.host_data().title_id
}
