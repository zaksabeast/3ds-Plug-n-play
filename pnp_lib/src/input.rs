use super::bindings;
use num_enum::IntoPrimitive;
use std::{
    cmp::PartialEq,
    ops::{BitAnd, BitOr},
};

/// A button that can be pressed by a user.
#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive)]
#[repr(u32)]
pub enum Button {
    A = 1,
    B = 2,
    Select = 4,
    Start = 8,
    Dright = 16,
    Dleft = 32,
    Dup = 64,
    Ddown = 128,
    R = 256,
    L = 512,
    X = 1024,
    Y = 2048,
}

impl PartialEq<Button> for u32 {
    fn eq(&self, other: &Button) -> bool {
        *self == *other as u32
    }
}

impl PartialEq<u32> for Button {
    fn eq(&self, other: &u32) -> bool {
        *self as u32 == *other
    }
}

impl BitAnd<Button> for u32 {
    type Output = u32;

    fn bitand(self, rhs: Button) -> Self::Output {
        self & (rhs as u32)
    }
}

impl BitOr for Button {
    type Output = u32;

    fn bitor(self, rhs: Self) -> Self::Output {
        (self as u32) | (rhs as u32)
    }
}

/// Get buttons that were just pressed.
/// This is great for getting buttons once and
/// doing multiple checks on the buttons.
///
/// # Examples
/// ```
/// use pnp::{Button, just_pressed};
///
/// let pressed = just_pressed();
/// if pressed == Button::Dup | Button::Ddown {
///   // Do something
/// } else if pressed == Button::Start {
///   // Do something else
/// }
/// ```
pub fn just_pressed() -> u32 {
    unsafe { bindings::host_just_pressed() }
}

/// Check if buttons were just pressed.
/// Convenient for one time checks.
///
/// # Examples
/// ```
/// use pnp::{Button, is_just_pressed};
///
/// if is_just_pressed(Button::Dup | Button::Ddown) {
///   // Do something
/// }
/// ```
pub fn is_just_pressed(io_bits: impl Into<u32>) -> bool {
    let is_pressed = unsafe { bindings::host_is_just_pressed(io_bits.into()) };
    is_pressed != 0
}
