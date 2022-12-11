use super::plugin_runner;
use ctr::res::CtrResult;

pub struct PnpServiceContext {
    pub is_paused: bool,
    pub plugin_runner: Option<plugin_runner::PluginRunner>,
}

impl PnpServiceContext {
    pub fn new() -> CtrResult<Self> {
        Ok(Self {
            is_paused: false,
            plugin_runner: None,
        })
    }
}
