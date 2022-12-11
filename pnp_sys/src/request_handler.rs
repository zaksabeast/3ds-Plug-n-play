use super::{
    display::Screen, frame_pause::handle_frame_pause, hook::get_running_title_id,
    memory::GameMemory, plugin_runner::PluginRunner,
};
use crate::{hook, PnpSysmodule};
use ctr::{ctr_method, res::CtrResult, sysmodule::server::Service};
use no_std_io::{EndianRead, EndianWrite};
use num_enum::{FromPrimitive, IntoPrimitive};

#[derive(IntoPrimitive, FromPrimitive)]
#[repr(u16)]
pub enum PnpGameCommand {
    #[num_enum(default)]
    InvalidCommand = 0,
    RunGameHook = 1,
}

impl Service for PnpGameCommand {
    const ID: usize = 0;
    const NAME: &'static str = "pnp:game";
    const MAX_SESSION_COUNT: i32 = 8;
}

#[derive(EndianRead, EndianWrite)]
struct RunGameHookIn {
    placeholder: u32,
    screen_id: u32,
    frame_buffer: u32,
    stride: u32,
    format: u32,
}

#[ctr_method(cmd = "PnpGameCommand::RunGameHook", normal = 0x1, translate = 0x0)]
fn run_game_hook(
    server: &mut PnpSysmodule,
    _session_index: usize,
    input: RunGameHookIn,
) -> CtrResult {
    if hook::is_new_game_launch() {
        // Explicitly drop the current runner for memory purposes
        server.context.plugin_runner = None;
        let title_id = get_running_title_id()?;
        let game = GameMemory::new(title_id)?;
        server.context.plugin_runner = PluginRunner::new(title_id, game);
    }

    let is_top_screen = input.screen_id == 0;
    let screen = Screen::new(
        is_top_screen,
        input.frame_buffer,
        input.stride,
        input.format,
    )?;

    if is_top_screen {
        if let Some(plugin_runner) = &mut server.context.plugin_runner {
            plugin_runner.run_frame(screen)?;
        }
    }

    handle_frame_pause(&mut server.context, is_top_screen);

    Ok(())
}
