#![no_std]
#![allow(incomplete_features)]
#![feature(start)]
#![feature(if_let_guard)]

extern crate alloc;

/// Sysmodule context, which persists data between frames.
mod context;
/// Everything related to drawing on the screen.
mod display;
/// Handles game pausing and frame advancing like an emulator to easily get frame accurate button presses.
mod frame_pause;
/// Tools for pnp to hook into games.
mod hook;
mod log;
/// Tools to interact with game memory.
mod memory;
/// Sysmodule notification handlers.
mod notification;
/// Handles loading and running plugins.
mod plugin_runner;
/// Sysmodule request handler.
mod request_handler;

use alloc::vec;
use context::PnpServiceContext;
use ctr::{
    allocator::mappable_init,
    ipc::WrittenCommand,
    match_ctr_route,
    res::CtrResult,
    srv,
    sysmodule::server::{Service, ServiceManager, ServiceRouter},
    thread,
};
use hook::set_pnp_session_handle;
use request_handler::PnpGameCommand;

struct PnpSysmodule {
    context: PnpServiceContext,
}

impl PnpSysmodule {
    fn new() -> Self {
        Self {
            context: PnpServiceContext::new().unwrap(),
        }
    }
}

impl ServiceRouter for PnpSysmodule {
    fn handle_request(
        &mut self,
        service_id: usize,
        session_index: usize,
    ) -> CtrResult<WrittenCommand> {
        match_ctr_route!(
            PnpSysmodule,
            service_id,
            session_index,
            PnpGameCommand::RunGameHook,
        )
    }

    fn accept_session(&mut self, _session_index: usize) {}
    fn close_session(&mut self, _session_index: usize) {}
}

#[cfg_attr(feature = "large_mem", ctr::ctr_start(heap_byte_size = 0x800000))]
#[cfg_attr(not(feature = "large_mem"), ctr::ctr_start(heap_byte_size = 0x550000))]
fn main() {
    mappable_init(0x10000000, 0x14000000);

    let router = PnpSysmodule::new();
    let services = vec![PnpGameCommand::register().unwrap()];
    let notification_manger = notification::init_manager().unwrap();
    let mut manager = ServiceManager::new(services, notification_manger, router);

    // Copy the session handle.
    // This has to be in a new thread since the main thread is handling requests.
    thread::spawn(|| {
        let handle = srv::get_service_handle_direct("pnp:game").unwrap();
        set_pnp_session_handle(handle);
        // Attempt to install the hook, in case this was
        // launched for an o3ds extended memory game.
        hook::install_hook();
    });

    if let Err(result_code) = manager.run() {
        let raw_code = result_code.into_raw();
        log::error(&alloc::format!("manager.run error {:x}", raw_code));
    };
}
