//!

use super::c_api;

/// A bitmask of mice input events.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NcMiceEvents(pub c_api::NcMiceEvents_u32);

/// # Constants
impl NcMiceEvents {
    /// [`NcMiceEvents`] flag that **disables all** mice events.
    pub const None: NcMiceEvents = Self(c_api::NCMICE_NO_EVENTS);

    /// [`NcMiceEvents`] flag that enables mice **move** events.
    pub const Move: NcMiceEvents = Self(c_api::NCMICE_MOVE_EVENTS);

    /// [`NcMiceEvents`] flag that enables mice **button** events.
    pub const Button: NcMiceEvents = Self(c_api::NCMICE_BUTTON_EVENTS);

    /// [`NcMiceEvents`] flag that enables mice **drag** events.
    pub const Drag: NcMiceEvents = Self(c_api::NCMICE_DRAG_EVENTS);

    /// [`NcMiceEvents`] flag that **enables all** mice tracking events.
    pub const All: NcMiceEvents = Self(c_api::NCMICE_ALL_EVENTS);
}

/// # Methods
impl NcMiceEvents {
    /// Returns a new `NcMiceEvents`.
    pub fn new(value: c_api::NcMiceEvents_u32) -> Self {
        Self(value)
    }

    /// Returns true if the current style has included the `other_style`.
    pub fn has(&self, other: NcMiceEvents) -> bool {
        (self.0 & other.0) == other.0
    }

    /// Adds the `other_style` to the current style.
    pub fn add(&mut self, other: NcMiceEvents) {
        self.0 |= other.0
    }
}

mod std_impl {
    use super::{c_api::NcMiceEvents_u32, NcMiceEvents};

    impl Default for NcMiceEvents {
        fn default() -> Self {
            Self::None
        }
    }

    crate::unit_impl_from![NcMiceEvents, NcMiceEvents_u32];

    crate::unit_impl_ops![bitwise; NcMiceEvents];
}
