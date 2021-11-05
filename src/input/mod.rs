//! `NcInput` & `NcKey`

// functions manually reimplemented: 4
// ------------------------------------------
// (+) done: 4
// (#) test: 0
// ------------------------------------------
// + ncinput_equal_p
// + ncinput_nomod_p
// + nckey_mouse_p
// + nckey_supppuab_p

use crate::NcDim;

pub(crate) mod reimplemented;

/// Reads and decodes input events.
///
/// Reads from stdin and decodes the input to stdout, including synthesized
/// events and mouse events. Notcurses provides input from keyboards and mice.
/// Single Unicode codepoints are received from the keyboard, directly encoded
/// as `u32`.
///
/// The input system must deal with numerous keyboard signals which do not map
/// to Unicode code points. This includes the keypad arrows and function keys.
/// These "synthesized" codepoints are enumerated in , and mapped into the
/// Supplementary Private Use Area-B (U+100000..U+10FFFD).
/// Mouse button events are similarly mapped into the SPUA-B.
///
/// All events carry a ncinput structure with them.
/// For mouse events, the x and y coordinates are reported within this struct.
/// For all events, modifiers (e.g. "Alt") are carried as bools in this struct.
pub type NcInput = crate::bindings::ffi::ncinput;

/// New NcInput.
impl NcInput {
    /// New empty NcInput.
    pub const fn new_empty() -> NcInput {
        NcInput {
            id: 0,
            y: 0,
            x: 0,
            alt: false,
            shift: false,
            ctrl: false,
            evtype: NcEvType::UNKNOWN,
            ypx: -1,
            xpx: -1,
        }
    }

    /// New NcInput.
    pub const fn new(id: char) -> NcInput {
        Self::with_all_args(id, None, None, false, false, false, 0)
    }

    /// New NcInput with alt key.
    pub const fn with_alt(id: char) -> NcInput {
        Self::with_all_args(id, None, None, true, false, false, 0)
    }

    /// New NcInput with shift key.
    pub const fn with_shift(id: char) -> NcInput {
        Self::with_all_args(id, None, None, false, true, false, 0)
    }

    /// New NcInput with ctrl key.
    pub const fn with_ctrl(id: char) -> NcInput {
        Self::with_all_args(id, None, None, false, false, true, 0)
    }

    /// New NcInput, expecting all the arguments.
    pub const fn with_all_args(
        id: char,
        x: Option<NcDim>,
        y: Option<NcDim>,
        alt: bool,
        shift: bool,
        ctrl: bool,
        evtype: NcEvType,
    ) -> NcInput {
        let (ix, iy);
        if let Some(x) = x {
            ix = x as i32
        } else {
            ix = -1
        };
        if let Some(y) = y {
            iy = y as i32
        } else {
            iy = -1
        };

        NcInput {
            id: id as u32,
            y: ix,
            x: iy,
            alt,
            shift,
            ctrl,
            evtype,
            ypx: -1,
            xpx: -1,
        }
    }
}

/// The type of the event, part of [`NcInput`] (alias of u32).
pub type NcEvType = u32;

crate::impl_api![
    NcEvType,
    NcEvTypeApi,
    /// *Unknown* type event ([`NcEvType`]).
    const UNKNOWN: NcEvType = constants::NCEVTYPE_UNKNOWN;,
    /// *Press* type event ([`NcEvType`]).
    const PRESS: NcEvType = constants::NCEVTYPE_PRESS;,
    /// *Repeat* type event ([`NcEvType`]).
    const REPEAT: NcEvType = constants::NCEVTYPE_REPEAT;,
    /// *Release* type event ([`NcEvType`]).
    const RELEASE: NcEvType = constants::NCEVTYPE_RELEASE;
];

/// A mask for mice input events (alias of u32).
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

pub(crate) mod constants {
    use crate::{NcEvType, NcMiceEvents};

    /// *Unknown* type event ([`NcEvType`]).
    pub const NCEVTYPE_UNKNOWN: NcEvType = crate::bindings::ffi::ncinput_NCTYPE_UNKNOWN;

    /// *Press* type event ([`NcEvType`]).
    pub const NCEVTYPE_PRESS: NcEvType = crate::bindings::ffi::ncinput_NCTYPE_PRESS;

    /// *Repeat* type event ([`NcEvType`]).
    pub const NCEVTYPE_REPEAT: NcEvType = crate::bindings::ffi::ncinput_NCTYPE_REPEAT;

    /// *Release* type event ([`NcEvType`]).
    pub const NCEVTYPE_RELEASE: NcEvType = crate::bindings::ffi::ncinput_NCTYPE_RELEASE;

    // Mice events:

    /// Disables all mice events.
    pub const NCMICE_NO_EVENTS: NcMiceEvents = crate::bindings::ffi::NCMICE_NO_EVENTS;

    /// Enables mice *move* events
    pub const NCMICE_MOVE_EVENTS: NcMiceEvents = crate::bindings::ffi::NCMICE_MOVE_EVENT;

    /// Enables mice *button** events
    pub const NCMICE_BUTTON_EVENTS: NcMiceEvents = crate::bindings::ffi::NCMICE_BUTTON_EVENT;

    /// Enables mice *drag* events
    pub const NCMICE_DRAG_EVENTS: NcMiceEvents = crate::bindings::ffi::NCMICE_DRAG_EVENT;

    /// Enables all mice events.
    pub const NCMICE_ALL_EVENTS: NcMiceEvents = crate::bindings::ffi::NCMICE_ALL_EVENTS;
}
