//!

/// A bitmask of mice input events.
///
/// # Default
/// *[`NcMiceEvents::None`]
///
/// # Flags
/// - [`Move`][NcMiceEvents::Move]
/// - [`Button`][NcMiceEvents::Button]
/// - [`Drag`][NcMiceEvents::Drag]
/// - [`None`][NcMiceEvents::None]
/// - [`All`][NcMiceEvents::All]
///
/// # Used by
/// - [`Nc.mice_disable`][crate::Nc#method.mice_disable]
/// - [`Nc.mice_enable`][crate::Nc#method.mice_enable]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NcMiceEvents(pub c_api::NcMiceEvents_u32);

/// # Flags
impl NcMiceEvents {
    /// Disables all mice events.
    pub const None: NcMiceEvents = Self(c_api::NCMICE_NO_EVENTS);

    /// Enables mice move events.
    pub const Move: NcMiceEvents = Self(c_api::NCMICE_MOVE_EVENTS);

    /// Enables mice button events.
    pub const Button: NcMiceEvents = Self(c_api::NCMICE_BUTTON_EVENTS);

    /// Enables mice drag events.
    pub const Drag: NcMiceEvents = Self(c_api::NCMICE_DRAG_EVENTS);

    /// Enables all mice tracking events.
    pub const All: NcMiceEvents = Self(c_api::NCMICE_ALL_EVENTS);
}

/// # Methods
impl NcMiceEvents {
    /// Returns a new `NcMiceEvents`.
    pub fn new(value: c_api::NcMiceEvents_u32) -> Self {
        Self(value)
    }

    /// Returns true if the current mice events has `other` included.
    pub fn has(&self, other: NcMiceEvents) -> bool {
        (self.0 & other.0) == other.0
    }

    /// Adds `other` to the current mice events.
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
    crate::from_primitive![NcMiceEvents, NcMiceEvents_u32];
    crate::unit_impl_from![NcMiceEvents, NcMiceEvents_u32];
    crate::unit_impl_ops![bitwise; NcMiceEvents, NcMiceEvents_u32];
    crate::unit_impl_fmt![bases+display; NcMiceEvents];
}

pub(crate) mod c_api {
    use crate::c_api::ffi;

    /// A bitmask for mice input events.
    ///
    /// It's recommended to use [`NcMiceEvents`][crate::NcMiceEvents] instead.
    ///
    /// # Associated `c_api` constants
    /// - [`NCMICE_NO_EVENTS`]
    /// - [`NCMICE_MOVE_EVENTS`]
    /// - [`NCMICE_BUTTON_EVENTS`]
    /// - [`NCMICE_DRAG_EVENTS`]
    /// - [`NCMICE_ALL_EVENTS`]
    pub type NcMiceEvents_u32 = u32;

    /// [`NcMiceEvents_u32`] flag that disables all mice events.
    pub const NCMICE_NO_EVENTS: NcMiceEvents_u32 = ffi::NCMICE_NO_EVENTS;

    /// [`NcMiceEvents_u32`] flag that enables mice *move* events
    pub const NCMICE_MOVE_EVENTS: NcMiceEvents_u32 = ffi::NCMICE_MOVE_EVENT;

    /// [`NcMiceEvents_u32`] flag that enables mice *button** events
    pub const NCMICE_BUTTON_EVENTS: NcMiceEvents_u32 = ffi::NCMICE_BUTTON_EVENT;

    /// [`NcMiceEvents_u32`] flag that enables mice *drag* events
    pub const NCMICE_DRAG_EVENTS: NcMiceEvents_u32 = ffi::NCMICE_DRAG_EVENT;

    /// [`NcMiceEvents_u32`] flag that enables all mice events.
    pub const NCMICE_ALL_EVENTS: NcMiceEvents_u32 = ffi::NCMICE_ALL_EVENTS;
}
