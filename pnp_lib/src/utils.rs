use super::bindings;

/// Returns the running title id.
pub fn title_id() -> u64 {
    unsafe { bindings::host_get_game_title_id() }
}
