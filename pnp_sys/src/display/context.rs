use super::{
    pixel::{draw_format_2, draw_other_format},
    Color, SCREEN_HEIGHT, SCREEN_WIDTH_TOP,
};
use core::fmt;
use ctr::{
    res::{error, CtrResult},
    Process,
};

const VRAM_ADDRESS_MIN: u32 = 0x1F000000;
const VRAM_ADDRESS_MAX: u32 = 0x1F5FFFFF;

const OLD_FCRAM_ADDRESS_MIN: u32 = 0x14000000;
const OLD_FCRAM_ADDRESS_MAX: u32 = 0x1C000000;

const NEW_FCRAM_ADDRESS_MIN: u32 = 0x30000000; // Uncached FCRAM
const NEW_FCRAM_ADDRESS_MAX: u32 = 0x3FFFFFFF; // Uncached FCRAM

const UNCACHED_FCRAM_ADDRESS_MIN: u32 = 0xA0000000; // Uncached FCRAM

pub struct ScreenContext {
    pub(super) is_top_screen: bool,
    pub(super) frame_buffer: u32,
    pub(super) stride: u32,
    pub(super) format: u32,
}

impl ScreenContext {
    fn addr_is_vram(addr: u32) -> bool {
        (VRAM_ADDRESS_MIN..=VRAM_ADDRESS_MAX).contains(&addr)
    }

    fn addr_is_old_fcram(addr: u32) -> bool {
        (OLD_FCRAM_ADDRESS_MIN..=OLD_FCRAM_ADDRESS_MAX).contains(&addr)
    }

    fn addr_is_new_fcram(addr: u32) -> bool {
        (NEW_FCRAM_ADDRESS_MIN..=NEW_FCRAM_ADDRESS_MAX).contains(&addr)
    }

    fn get_writable_addr(addr: u32) -> CtrResult<u32> {
        if Self::addr_is_vram(addr) {
            return Ok(addr);
        }

        if Self::addr_is_old_fcram(addr) {
            return Ok(UNCACHED_FCRAM_ADDRESS_MIN | (addr & 0xffffff));
        }

        if Self::addr_is_new_fcram(addr) {
            return Ok(UNCACHED_FCRAM_ADDRESS_MIN | (addr & 0xfffffff));
        }

        Err(error::invalid_pointer())
    }

    pub fn new(is_top_screen: bool, addr: u32, stride: u32, format: u32) -> CtrResult<Self> {
        Ok(Self {
            frame_buffer: Self::get_writable_addr(addr)?,
            is_top_screen,
            stride,
            format,
        })
    }

    pub fn flush(&self) -> CtrResult {
        // The uncached fcram doesn't need flushing
        if Self::addr_is_vram(self.frame_buffer) {
            let flush_size = (SCREEN_HEIGHT * SCREEN_WIDTH_TOP * 4) as usize;
            Process::current().flush_process_data_cache(self.frame_buffer, flush_size)?;
        }

        Ok(())
    }

    /// # Safety
    /// The caller needs to make sure:
    /// - x is never above 320 for a bottom screen
    /// - x is never above 340 for a top screen
    /// - y is never above 240
    #[inline(always)]
    pub unsafe fn draw_pixel(&self, color: &Color, x: u32, y: u32) {
        if self.format & 0xf == 2 {
            draw_format_2(color, self.frame_buffer, self.stride, x, y)
        } else {
            draw_other_format(color, self.frame_buffer, self.stride, x, y)
        }
    }
}

impl fmt::Debug for ScreenContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ScreenContext")
            .field("is_top_screen", &self.is_top_screen)
            .field("addr", &self.frame_buffer)
            .field("stride", &self.stride)
            .finish()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod new {
        use super::*;

        #[test]
        fn should_error_if_the_address_is_not_in_an_acceptable_range() {
            let result = ScreenContext::new(true, 0, 0, 0).unwrap_err();
            assert_eq!(result, error::invalid_pointer());
        }

        #[test]
        fn should_succeed_if_the_address_is_vram() {
            ScreenContext::new(true, VRAM_ADDRESS_MIN, 0, 0).unwrap();
        }

        #[test]
        fn should_succeed_if_the_address_is_old_fcram() {
            ScreenContext::new(true, OLD_FCRAM_ADDRESS_MIN, 0, 0).unwrap();
        }

        #[test]
        fn should_succeed_if_the_address_is_new_fcram() {
            ScreenContext::new(true, NEW_FCRAM_ADDRESS_MIN, 0, 0).unwrap();
        }

        #[test]
        fn old_fcram_should_use_uncached_addr() {
            let context = ScreenContext::new(true, OLD_FCRAM_ADDRESS_MIN, 0, 0).unwrap();
            assert_eq!(context.frame_buffer, UNCACHED_FCRAM_ADDRESS_MIN)
        }

        #[test]
        fn new_fcram_should_use_uncached_addr() {
            let context = ScreenContext::new(true, NEW_FCRAM_ADDRESS_MIN, 0, 0).unwrap();
            assert_eq!(context.frame_buffer, UNCACHED_FCRAM_ADDRESS_MIN)
        }
    }
}
