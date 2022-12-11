use super::circular_counter::CircularCounter;
use alloc::{format, string::String, vec::Vec};

pub struct Menu {
    counter: CircularCounter,
    options: Vec<String>,
}

impl Menu {
    fn new_counter(options: &[String]) -> CircularCounter {
        CircularCounter::new(0, options.len() - 1)
    }

    pub fn new(options: Vec<String>) -> Self {
        Self {
            counter: Self::new_counter(&options),
            options,
        }
    }

    pub fn value(&self) -> &str {
        &self.options[self.counter.value()]
    }

    fn cursor_str(&self, index: usize) -> &str {
        if self.counter.value() == index {
            ">"
        } else {
            " "
        }
    }

    pub fn push_menu_to_buffer(&self, buf: &mut Vec<String>) {
        for (index, option) in self.options.iter().enumerate() {
            buf.push(format!("{} {}", self.cursor_str(index), option));
        }
    }

    pub fn cursor_down(&mut self) {
        self.counter.increment();
    }

    pub fn cursor_up(&mut self) {
        self.counter.decrement();
    }
}
