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

use std::ffi::CStr;

use crate::{NcDim, NcKey};

pub(crate) mod reimplemented;

/// A received character or event.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NcReceived {
    /// A valid [`char`] was received.
    Char(char),
    /// A synthesized event was received.
    Event(NcKey),
    /// No input was received
    ///
    /// A `0x00` (NUL) was received, meaning no input.
    NoInput,
    /// Something other was received.
    Other(u32),
}

impl NcReceived {
    /// New `NcReceived`, from a `u32` number.
    pub fn new(num: u32) -> Self {
        if num == 0 {
            Self::NoInput
        } else if NcKey::is(num) {
            Self::Event(NcKey::new(num).unwrap())
        } else if let Some(c) = core::char::from_u32(num) {
            Self::Char(c)
        } else {
            Self::Other(num)
        }
    }
}

impl From<NcInput> for NcReceived {
    fn from(i: NcInput) -> Self {
        Self::new(i.id)
    }
}

impl From<NcReceived> for u32 {
    fn from(r: NcReceived) -> Self {
        use NcReceived::*;
        match r {
            Char(c) => c.into(),
            Event(e) => e.into(),
            NoInput => 0,
            Other(o) => o,
        }
    }
}

/// Reads and decodes input events.
///
/// Reads from stdin and decodes the input to stdout, including synthesized
/// events and mouse events. Notcurses provides input from keyboards and mice.
/// Single Unicode codepoints are received from the keyboard, directly encoded
/// as `u32`.
///
/// All events carry an `NcInput` structure with them.
///
/// For mouse events, the x and y coordinates are reported within this struct.
/// For all events, modifiers (e.g. "Alt") are carried as bools in this struct.
pub type NcInput = crate::bindings::ffi::ncinput;

/// # Constructors
impl NcInput {
    /// New empty `NcInput`.
    pub const fn new_empty() -> NcInput {
        NcInput {
            id: 0,
            y: 0,
            x: 0,
            utf8: [0; 5],
            alt: false,
            shift: false,
            ctrl: false,
            evtype: NcEvType::UNKNOWN,
            ypx: -1,
            xpx: -1,
        }
    }

    /// New `NcInput`.
    pub const fn new(id: char) -> NcInput {
        Self::with_all_args(id, None, None, false, false, false, 0)
    }

    /// New `NcInput` with `alt` key.
    pub const fn with_alt(id: char) -> NcInput {
        Self::with_all_args(id, None, None, true, false, false, 0)
    }

    /// New `NcInput` with `shift` key.
    pub const fn with_shift(id: char) -> NcInput {
        Self::with_all_args(id, None, None, false, true, false, 0)
    }

    /// New `NcInput` with `ctrl` key.
    pub const fn with_ctrl(id: char) -> NcInput {
        Self::with_all_args(id, None, None, false, false, true, 0)
    }

    /// New `NcInput`, expecting all the arguments (except utf8).
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
            utf8: [0; 5],
            alt,
            shift,
            ctrl,
            evtype,
            ypx: -1,
            xpx: -1,
        }
    }
}

/// # Methods
impl NcInput {
    /// Returns the `char` from the utf8 representation of the input.
    pub fn char(&self) -> Option<char> {
        let cstr = unsafe { CStr::from_ptr(self.utf8.as_ptr()) };
        let string = cstr.to_string_lossy();
        let raw_char = string.chars().next();
        if let Some(ch) = raw_char {
            if ch.is_ascii_control() {
                None
            } else {
                Some(ch)
            }
        } else {
            None
        }
    }
}

/// The type of the event, part of [`NcInput`] (alias of `u32`).
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
