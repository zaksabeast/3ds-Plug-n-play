use super::{context, font, Color, SCREEN_HEIGHT, SCREEN_WIDTH_BOTTOM, SCREEN_WIDTH_TOP};
use ctr::res::{error, CtrResult};

pub struct Screen {
    context: context::ScreenContext,
}

impl Screen {
    pub fn new(is_top_screen: bool, addr: u32, stride: u32, format: u32) -> CtrResult<Self> {
        Ok(Self {
            context: context::ScreenContext::new(is_top_screen, addr, stride, format)?,
        })
    }

    pub fn get_is_top_screen(&self) -> bool {
        self.context.is_top_screen
    }

    /// # Safety
    /// The caller needs to make sure:
    /// - The x is never above 320 for a bottom screen
    /// - The x is never above 340 for a top screen
    /// - The y is never above 240
    #[inline(always)]
    unsafe fn draw_pixel(&self, color: &Color, x: u32, y: u32) {
        self.context.draw_pixel(color, x, y)
    }

    #[inline(always)]
    fn is_safe_pixel(&self, x: u32, y: u32) -> bool {
        if y > SCREEN_HEIGHT {
            return false;
        }

        let is_top_screen = self.get_is_top_screen();

        if is_top_screen && x > SCREEN_WIDTH_TOP {
            return false;
        }

        if !is_top_screen && x > SCREEN_WIDTH_BOTTOM {
            return false;
        }

        true
    }

    #[inline(always)]
    fn is_safe_pixel_range(&self, x1: u32, y1: u32, x2: u32, y2: u32) -> bool {
        self.is_safe_pixel(x1, y1) && self.is_safe_pixel(x2, y2)
    }

    unsafe fn draw_character(&self, color: &Color, letter: char, x: u32, y: u32) -> CtrResult<()> {
        let font_char = font::convert_letter_to_font(letter);
        let mask = 0b10000000;

        for (y_offset, draw_line) in font_char.iter().enumerate() {
            for x_offset in 0..font::CHAR_WIDTH {
                if ((mask >> x_offset) & *draw_line) != 0 {
                    self.draw_pixel(color, x_offset + x, (y_offset as u32) + y);
                }
            }
        }

        Ok(())
    }

    pub fn draw_string(&self, color: &Color, text: &str, x: u32, y: u32) -> CtrResult<()> {
        let text_len = text.len() as u32;
        if !self.is_safe_pixel_range(
            x,
            y,
            x + (text_len * font::CHAR_WIDTH),
            y + font::CHAR_HEGHT,
        ) {
            return Err(error::invalid_value());
        }

        for (index, letter) in text.chars().enumerate() {
            // This is safe because the pixels are validated ahead of time
            unsafe { self.draw_character(color, letter, x + (index * 8) as u32, y) }?;
        }

        Ok(())
    }

    pub fn draw_square(
        &self,
        color: &Color,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
    ) -> CtrResult<()> {
        let x_max = x + width;
        let y_max = y + height;

        if !self.is_safe_pixel_range(x, y, x_max, y_max) {
            return Err(error::invalid_value());
        }

        for current_x in x..x_max {
            for current_y in y..y_max {
                // This is safe because the pixels are validated ahead of time
                unsafe { self.draw_pixel(color, current_x, current_y) };
            }
        }

        Ok(())
    }

    pub fn flush(&self) -> CtrResult {
        self.context.flush()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod is_safe_pixel {
        use super::*;

        #[test]
        fn should_return_false_if_the_y_is_too_large() {
            let screen = Screen::new(true, 0x1F000000, 0, 0).unwrap();
            let result = screen.is_safe_pixel(0, SCREEN_HEIGHT + 1);
            assert_eq!(result, false);
        }

        #[test]
        fn should_return_false_if_the_x_is_too_large_for_the_top_screen() {
            let screen = Screen::new(true, 0x1F000000, 0, 0).unwrap();
            let result = screen.is_safe_pixel(SCREEN_WIDTH_TOP + 1, 0);
            assert_eq!(result, false);
        }

        #[test]
        fn should_return_false_if_the_x_is_too_large_for_the_bottom_screen() {
            let screen = Screen::new(false, 0x1F000000, 0, 0).unwrap();
            let result = screen.is_safe_pixel(SCREEN_WIDTH_BOTTOM + 1, 0);
            assert_eq!(result, false);
        }

        #[test]
        fn should_return_true_if_the_coordinates_are_under_the_max_sizes_for_the_top_screen() {
            let screen = Screen::new(true, 0x1F000000, 0, 0).unwrap();
            let result = screen.is_safe_pixel(SCREEN_WIDTH_TOP, SCREEN_HEIGHT);
            assert_eq!(result, true);
        }

        #[test]
        fn should_return_true_if_the_coordinates_are_under_the_max_sizes_for_the_bottom_screen() {
            let screen = Screen::new(false, 0x1F000000, 0, 0).unwrap();
            let result = screen.is_safe_pixel(SCREEN_WIDTH_BOTTOM, SCREEN_HEIGHT);
            assert_eq!(result, true);
        }
    }
}
