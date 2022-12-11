// Thanks to NTR for the draw functions - https://github.com/44670/ntr_overlay_samples/blob/5fee35f160190fbcf0eddb54143c1bfd27b2586f/fps/source/ov.c

use super::Color;

/// # Safety
/// The caller needs to make sure:
/// - x is never above 320 for a bottom screen
/// - x is never above 340 for a top screen
/// - y is never above 240
pub unsafe fn draw_format_2(color: &Color, frame_buffer: u32, stride: u32, x: u32, y: u32) {
    let pixel = (((color.r & 0x1f) as u16) << 11)
        | (((color.g & 0x3f) as u16) << 5)
        | ((color.b & 0x1f) as u16);
    let vram = (frame_buffer + (stride * x) + 480 - (2 * y)) as *mut u16;
    vram.write(pixel);
}

/// # Safety
/// The caller needs to make sure:
/// - x is never above 320 for a bottom screen
/// - x is never above 340 for a top screen
/// - y is never above 240
pub unsafe fn draw_other_format(color: &Color, frame_buffer: u32, stride: u32, x: u32, y: u32) {
    let vram = (frame_buffer + (stride * x) - (3 * y)) as *mut u8;
    vram.write(color.b);
    vram.add(1).write(color.g);
    vram.add(2).write(color.r);
}
