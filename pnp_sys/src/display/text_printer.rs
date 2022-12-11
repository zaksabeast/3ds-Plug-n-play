use crate::{
    display,
    display::{Color, Screen},
};
use core::cmp;
use ctr::res::CtrResult;

pub struct TextPrinter {
    print_x: u32,
    print_y: u32,
    max_len: u8,
    text_color: Color,
    background_color: Color,
}

impl TextPrinter {
    pub fn new_with_max_len(max_len: u8) -> Self {
        Self {
            max_len,
            ..Default::default()
        }
    }

    pub fn draw_to_screen<T: AsRef<str>>(
        &mut self,
        screen: &Screen,
        print_lines: &[T],
    ) -> CtrResult {
        if !print_lines.is_empty() {
            screen.draw_square(
                &self.background_color,
                self.print_x,
                self.print_y,
                (self.max_len as u32).saturating_mul(8).saturating_add(8),
                (print_lines.len() as u32)
                    .saturating_mul(12)
                    .saturating_add(4),
            )?;
        }

        let text_x = self.print_x.saturating_add(4);
        let text_y = self.print_y.saturating_add(4);

        for (print_line, text) in print_lines.iter().enumerate() {
            let print_line_y = (print_line as u32)
                .saturating_mul(12)
                .saturating_add(text_y);

            let text_str = text.as_ref();
            let text_end = cmp::min(self.max_len as usize, text_str.len());

            screen.draw_string(
                &self.text_color,
                &text_str[0..text_end],
                text_x,
                print_line_y,
            )?;
        }

        Ok(())
    }

    pub fn set_print_x(&mut self, print_x: u32) {
        self.print_x = print_x
    }

    pub fn set_print_y(&mut self, print_y: u32) {
        self.print_y = print_y
    }

    pub fn set_max_len(&mut self, max_len: u8) {
        self.max_len = max_len
    }

    pub fn set_text_color(&mut self, text_color: Color) {
        self.text_color = text_color
    }

    pub fn set_background_color(&mut self, background_color: Color) {
        self.background_color = background_color
    }
}

impl Default for TextPrinter {
    fn default() -> Self {
        Self {
            print_x: 8,
            print_y: 10,
            max_len: 30,
            text_color: display::WHITE,
            background_color: display::BLACK,
        }
    }
}
