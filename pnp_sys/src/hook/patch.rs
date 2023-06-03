use super::{get_pnp_session_handle, get_running_title_id};
use crate::{log, memory::GameMemRegion};
use core::sync::atomic::{AtomicBool, Ordering};
use ctr::{
    res::{error, CtrResult},
    Handle, Process,
};
use no_std_io::Reader;

// Present framebuffer uses different registers in different games.
// These instructions seem to be consistent, and are 2 instructions (8 bytes) into the function.
const PRESENT_FRAMEBUFFER_BYTES: [u8; 0x10] = [
    0x28, 0x00, 0x8d, 0xe2, 0x00, 0x80, 0xa0, 0xe3, 0x01, 0x70, 0xa0, 0xe1, 0x00, 0x0e, 0x90, 0xe8,
];

#[rustfmt::skip]
const HOOK: [u8; 0x94] = [
    0xf0, 0x5f, 0x2d, 0xe9, // stmdb      sp!,{r4 r5 r6 r7 r8 r9 r10 r11 r12 lr}
    0x0f, 0x00, 0x2d, 0xe9, // stmdb      sp!,{r0 r1 r2 r3}
    0xf0, 0x00, 0xbd, 0xe8, // ldmia      sp!,{r4 r5 r6 r7}
    0x28, 0x00, 0x8d, 0xe2, // add        r0,sp,#0x28
    0x00, 0x0e, 0x90, 0xe8, // ldmia      r0,{r9 r10 r11}
    // -------------------------------------------------------------------------------------
    // Injected reader call
    // -------------------------------------------------------------------------------------
    0x0f, 0x80, 0xa0, 0xe1, // cpy        r8,pc
    0x6c, 0x80, 0x88, 0xe2, // add        r8,r8,#0x6c
    0x07, 0x00, 0x98, 0xe8, // ldmia      r8,{r0 r1 r2}
    0x70, 0x8f, 0x1d, 0xee, // mrc        p15,0x0,r8,cr13,cr0,0x3
    0x98, 0x80, 0x88, 0xe2, // add        r8,r8,#0x98
    0x56, 0x06, 0x08, 0xe9, // stmdb      r8,{r1 r2 r4 r6 r9 r10}
    0x32, 0x00, 0x00, 0xef, // swi        0x32
    // -------------------------------------------------------------------------------------
    0xd4, 0x03, 0x00, 0xeb, // bl         get_screen
    0x5c, 0x10, 0x80, 0xe2, // add        r1,r0,#0x5c
    0x04, 0x21, 0x91, 0xe7, // ldr        r2,[r1,r4,lsl #0x2]
    0x04, 0x30, 0xa0, 0xe3, // mov        r3,#0x4
    0x00, 0x00, 0xd2, 0xe5, // ldrb       r0,[r2,#0x0]
    0x01, 0x00, 0x60, 0xe2, // rsb        r0,r0,#0x1
    0xff, 0x00, 0x00, 0xe2, // and        r0,r0,#0xff
    0x80, 0xe1, 0x60, 0xe0, // rsb        lr,r0,r0, lsl #0x3
    0x0e, 0x31, 0x83, 0xe0, // add        r3,r3,lr, lsl #0x2
    0x03, 0x30, 0x82, 0xe0, // add        r3,r2,r3
    0xe0, 0x0e, 0x83, 0xe8, // stmia      r3,{ r5 r6 r7 r9 r10 r11 }
    0x9a, 0x8f, 0x07, 0xee, // mcr        p15,0x0,r8,cr7,cr10,0x4
    0x04, 0x21, 0x91, 0xe7, // ldr        r2,[r1,r4,lsl #0x2]
    0x9f, 0x3f, 0x92, 0xe1, // ldrex      r3,[r2]
    0xff, 0x30, 0xc3, 0xe3, // bic        r3,r3,#0xff
    0x00, 0x30, 0x83, 0xe1, // orr        r3,r3,r0
    0xff, 0x3c, 0xc3, 0xe3, // bic        r3,r3,#0xff00
    0x01, 0x3c, 0x83, 0xe3, // orr        r3,r3,#0x100
    0x93, 0x6f, 0x82, 0xe1, // strex      r6,r3,[r2]
    0x00, 0x00, 0x56, 0xe3, // cmp        r6,#0x0
    0xf6, 0xff, 0xff, 0x1a, // bne        loop
    0xf0, 0x9f, 0xbd, 0xe8, // ldmia      sp!,{r4 r5 r6 r7 r8 r9 r10 r11 r12 pc}
    0x00, 0x00, 0x00, 0x00, // *session handle
    0x40, 0x01, 0x01, 0x00, // *command header
    0x00, 0x00, 0x00, 0x00, // unused
];

fn patch_present_framebuffer(title_id: u64, pnp_handle: Handle) -> CtrResult<()> {
    let game_memory = GameMemRegion::new_code(title_id)?;
    let mut hook_code: [u8; 0x94] = HOOK;

    let present_framebuffer = game_memory
        .find_pattern(&PRESENT_FRAMEBUFFER_BYTES)
        .ok_or_else(error::invalid_pointer)?;
    // Subtract 8 since the present framebuffer bytes are 2 instructions into the function.
    let present_framebuffer_offset = present_framebuffer.offset() - 8;

    let get_screen_branch = game_memory
        .slice()
        .read_le::<u32>(present_framebuffer_offset + 0x20)?;

    // Rebase get_screen branch
    // Original offset: 0x20
    // New offset: 0x30
    // 0x10 bytes difference / 0x4 bytes per instruction = 0x4 instructions
    let rebased_branch = get_screen_branch - 4;
    hook_code[0x30..0x34].copy_from_slice(&rebased_branch.to_le_bytes());

    // Patch session handle
    let raw_handle = unsafe { pnp_handle.get_raw() };
    hook_code[0x88..0x8c].copy_from_slice(&raw_handle.to_le_bytes());

    let game_code = game_memory.slice_mut();
    let start = present_framebuffer_offset;
    let end = present_framebuffer_offset + 0x94;
    let present_frambuffer_code = &mut game_code[start..end];

    // Write infinite loop to the frambuffer so the game doesn't execute
    present_frambuffer_code[..4].copy_from_slice(&0xeafbada0u32.to_le_bytes());
    // Write the hook after the infinite loop
    present_frambuffer_code[4..].copy_from_slice(&hook_code[4..]);
    // Write the hook over the infinite loop so the game executes
    present_frambuffer_code[..4].copy_from_slice(&hook_code[..4]);

    Ok(())
}

fn patch_running_title() -> CtrResult<()> {
    let title_id = get_running_title_id()?;
    let process = Process::new_from_title_id(title_id)?;
    let pnp_session_handle = get_pnp_session_handle();
    let handle_copy = process.copy_handle_to_process(&pnp_session_handle)?;

    patch_present_framebuffer(title_id, handle_copy)?;

    Ok(())
}

static IS_NEW_GAME_LAUNCH: AtomicBool = AtomicBool::new(false);

/// Determines if a game was just launched.
/// After this has been called once, it will always return `false` until a new game is launched.
pub fn is_new_game_launch() -> bool {
    IS_NEW_GAME_LAUNCH.swap(false, Ordering::Relaxed)
}

/// Attempts to install a hook.
/// Writes an error log if the hook failed.
/// Does not return a result, because there isn't anything to attempt
/// or recover from if this fails.
pub fn install_hook() {
    let patch_result = patch_running_title();

    // Don't forward the error if one exists
    if let Err(result_code) = patch_result {
        log::error(&alloc::format!(
            "Failed to hook title {:x}",
            u32::from(result_code)
        ));
    } else {
        IS_NEW_GAME_LAUNCH.store(true, Ordering::Relaxed);
    };
}
