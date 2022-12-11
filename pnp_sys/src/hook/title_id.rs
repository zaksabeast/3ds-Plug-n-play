pub use ctr::{pm_dbg, res::CtrResult};

pub fn get_running_title_id() -> CtrResult<u64> {
    let title_id = pm_dbg::get_current_app_info()?.program_info.program_id;
    Ok(title_id)
}
