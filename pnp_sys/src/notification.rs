use super::hook;
use ctr::{
    ptm_sysm,
    res::CtrResult,
    svc,
    sysmodule::notification::{NotificationHandlerResult, NotificationManager},
};

pub fn handle_launch_title_notification(_notification_id: u32) -> CtrResult {
    // Delay slightly so the game has time to load before patching.
    // In the future this should be replaced with reading the game's memory.
    svc::sleep_thread(1000000000);
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
