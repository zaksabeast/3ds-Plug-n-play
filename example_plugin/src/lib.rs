#[no_mangle]
pub extern "C" fn run_frame() {
    pnp::set_print_max_len(25);
    pnp::println!("Wasm plugin example");
    pnp::println!("Playing {:016x}", pnp::title_id());
}
