use super::context::PnpServiceContext;
use ctr::{hid, hid::InterfaceDevice, svc};

pub fn handle_frame_pause(context: &mut PnpServiceContext, is_bottom_screen: bool) {
    if hid::Global::is_just_pressed(hid::Button::Start | hid::Button::Select) {
        context.is_paused = true;
    }

    // Handle inputs on bottom screen since that seems to be rendered first.
    // This will prevent memory from changing too much in between pnp renders.
    while context.is_paused && is_bottom_screen {
        hid::Global::scan_input();

        let just_down = hid::Global::just_down_buttons();

        if just_down.select() {
            break;
        }

        if just_down.a() || just_down.start() {
            context.is_paused = false;
            break;
        }

        svc::sleep_thread(50000000);
    }
}
