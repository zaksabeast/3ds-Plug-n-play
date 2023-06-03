use super::hook;
use crate::{hook::get_running_title_id, memory::GameMemRegion};
use ctr::{
    ptm_sysm,
    res::CtrResult,
    svc,
    sysmodule::notification::{NotificationHandlerResult, NotificationManager},
};

// Arbitrary limit to prevent an infinite loop
const MAX_ATTEMPTS: usize = 100000;

pub fn handle_launch_title_notification(_notification_id: u32) -> CtrResult {
    let mut title_id = 0;

    // Wait until title id is accessible
    for _ in 0..MAX_ATTEMPTS {
        if let Ok(running_title_id) = get_running_title_id() {
            title_id = running_title_id;
            break;
        }
    }

    // Wait until game memory is accessible
    for _ in 0..MAX_ATTEMPTS {
        if GameMemRegion::new_heap(title_id).is_ok() {
            break;
        }
    }

    hook::install_hook();
    Ok(())
}

/// The notification Id is currently a u32 to avoid assumptions about the notifications that might be sent.
///
/// However it's probably safe to assume only [0x100, 0x179](https://github.com/LumaTeam/Luma3DS/blob/ebeef7ab7f730ae35658b66ca97c5da9f663a17d/sysmodules/loader/source/service_manager.c#L58-L59), and subscribed notifications will be used here, so an enum may be better here in the future.
fn handle_sleep_notification(notification_id: u32) -> NotificationHandlerResult {
    let _session = ptm_sysm::Session::new()?;

    if notification_id == ptm_sysm::NotificationId::SleepRequested {
        // Sleeping and logging seem to interfere with each other,
        // so we deny sleeping when in dev mode
        #[cfg(debug_assertions)]
        ptm_sysm::sys_reply_to_sleep_query(true)?;

        #[cfg(not(debug_assertions))]
        ptm_sysm::sys_reply_to_sleep_query(false)?;
    } else {
        let ack_value = ptm_sysm::sys_get_notification_ack_value(notification_id);
        ptm_sysm::sys_notify_sleep_preparation_complete(ack_value)?;
    }

    Ok(())
}

pub fn init_manager() -> CtrResult<NotificationManager> {
    let mut notification_manger = NotificationManager::new()?;

    notification_manger.subscribe(
        ptm_sysm::NotificationId::SleepRequested,
        handle_sleep_notification,
    )?;
    notification_manger.subscribe(
        ptm_sysm::NotificationId::GoingToSleep,
        handle_sleep_notification,
    )?;
    notification_manger.subscribe(
        ptm_sysm::NotificationId::FullyWakingUp,
        handle_sleep_notification,
    )?;
    notification_manger.subscribe(
        ptm_sysm::NotificationId::LaunchApp,
        handle_launch_title_notification,
    )?;
    notification_manger.subscribe(ptm_sysm::NotificationId::Termination, |_| {
        ctr::svc::exit_process();
    })?;

    Ok(notification_manger)
}
