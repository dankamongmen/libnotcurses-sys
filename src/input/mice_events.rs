//!

use super::constants;

/// A mask for mice input events (alias of `u32`).
pub type NcMiceEvents = u32;

crate::impl_api![
    NcMiceEvents,
    NcMiceEventsApi,
    /// [`NcMiceEvents`] flag that **disables all** mice events.
    const NO_EVENTS: NcMiceEvents = constants::NCMICE_NO_EVENTS;,
    /// [`NcMiceEvents`] flag that enables mice **move** events.
    const MOVE_EVENTS: NcMiceEvents = constants::NCMICE_MOVE_EVENTS;,
    /// [`NcMiceEvents`] flag that enables mice **button** events.
    const BUTTON_EVENTS: NcMiceEvents = constants::NCMICE_BUTTON_EVENTS;,
    /// [`NcMiceEvents`] flag that enables mice **drag** events.
    const DRAG_EVENTS: NcMiceEvents = constants::NCMICE_DRAG_EVENTS;,
    /// [`NcMiceEvents`] flag that **enables all** mice tracking events.
    const ALL_EVENTS: NcMiceEvents = constants::NCMICE_ALL_EVENTS;
];
