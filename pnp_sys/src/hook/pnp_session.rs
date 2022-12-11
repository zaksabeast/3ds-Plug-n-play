use core::{
    mem,
    sync::atomic::{AtomicU32, Ordering},
};
use ctr::Handle;

static PNP_HANDLE: AtomicU32 = AtomicU32::new(0);

/// Returns a pnp:game session handle.
/// This is manually dropped to avoid closing the session handle.
pub(super) fn get_pnp_session_handle() -> mem::ManuallyDrop<Handle> {
    let raw_handle = PNP_HANDLE.load(Ordering::Relaxed);
    let handle = raw_handle.into();
    mem::ManuallyDrop::new(handle)
}

/// Sets a pnp:game session handle.
pub fn set_pnp_session_handle(handle: Handle) {
    let raw_handle = unsafe { handle.get_raw() };
    PNP_HANDLE.store(raw_handle, Ordering::Relaxed);
    // The handle will be used through PNP_HANDLE
    #[allow(unused_must_use)]
    mem::ManuallyDrop::new(handle);
}
