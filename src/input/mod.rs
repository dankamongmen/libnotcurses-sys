//! `NcInput`

// functions manually reimplemented: 6
// ------------------------------------------
// (+) done: 6
// (W) wrap: 6
// (#) test: 0
// ------------------------------------------
//W+ ncinput_nomod_p
//W+ ncinput_shift_p
//W+ ncinput_ctrl_p
//W+ ncinput_alt_p
//W+ ncinput_meta_p
//W+ ncinput_equal_p

use std::ffi::CStr;

use crate::{c_api, NcDim, NcKey};

pub(crate) mod reimplemented;

mod input_type;
pub use input_type::NcInputType;

mod mice_events;
pub use mice_events::{NcMiceEvents, NcMiceEventsApi};

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
impl From<&NcInput> for NcReceived {
    fn from(i: &NcInput) -> Self {
        Self::new(i.id)
    }
}
impl From<&mut NcInput> for NcReceived {
    fn from(i: &mut NcInput) -> Self {
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
///
/// Single Unicode codepoints are received from the keyboard, directly encoded
/// as `u32`.
///
/// All events carry an `NcInput` structure with them.
///
/// For mouse events, the x and y coordinates are reported within this struct.
/// For all events, modifiers (e.g. "Alt") are carried as bools in this struct.
///
//
// WIP:
//
// An input event. Cell coordinates are currently defined only for mouse
// events. It is not guaranteed that we can set the modifiers for a given
// ncinput.
//
// We encompass single Unicode codepoints, not complete EGCs.
pub type NcInput = crate::bindings::ffi::ncinput;

impl PartialEq for NcInput {
    fn eq(&self, other: &Self) -> bool {
        self.equal_p(other)
    }
}

/// # Constructors
impl NcInput {
    /// New empty `NcInput`.
    pub const fn new_empty() -> NcInput {
        NcInput {
            id: 0,
            y: 0,
            x: 0,
            utf8: [0; 5],
            // TODO: DEPRECATED: do not use! going away in nc-4.0
            alt: false,
            shift: false,
            ctrl: false,
            // END DEPRECATION
            evtype: NcInputType::Unknown as u32,
            ypx: -1,
            xpx: -1,
            modifiers: 0,
        }
    }

    /// New `NcInput`.
    pub const fn new(id: char) -> NcInput {
        Self::with_all_args(id, None, None, false, false, false, NcInputType::Unknown)
    }

    /// New `NcInput` with `alt` key.
    pub const fn with_alt(id: char) -> NcInput {
        Self::with_all_args(id, None, None, true, false, false, NcInputType::Unknown)
    }

    /// New `NcInput` with `shift` key.
    pub const fn with_shift(id: char) -> NcInput {
        Self::with_all_args(id, None, None, false, true, false, NcInputType::Unknown)
    }

    /// New `NcInput` with `ctrl` key.
    pub const fn with_ctrl(id: char) -> NcInput {
        Self::with_all_args(id, None, None, false, false, true, NcInputType::Unknown)
    }

    /// New `NcInput`, expecting all the arguments (except utf8).
    pub const fn with_all_args(
        id: char,
        x: Option<NcDim>,
        y: Option<NcDim>,
        alt: bool,
        shift: bool,
        ctrl: bool,
        evtype: NcInputType,
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
            evtype: evtype as u32,
            ypx: -1,
            xpx: -1,
            modifiers: 0,
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

    /// Returns true if there are no modifiers present.
    ///
    /// *C style function: [ncinput_nomod_p()][c_api::ncinput_nomod_p].*
    pub const fn nomod_p(&self) -> bool {
        c_api::ncinput_nomod_p(self)
    }

    /// Returns true if the [`Shift`][crate::NcKeyMod::Shift] modifier is present.
    ///
    /// *C style function: [ncinput_shift_p()][c_api::ncinput_shift_p].*
    pub const fn shift_p(&self) -> bool {
        c_api::ncinput_shift_p(self)
    }

    /// Returns true if the [`Alt`][crate::NcKeyMod::Alt] modifier is present.
    ///
    /// *C style function: [ncinput_alt_p()][c_api::ncinput_alt_p].*
    pub const fn alt_p(&self) -> bool {
        c_api::ncinput_alt_p(self)
    }

    /// Returns true if the [`Ctrl`][crate::NcKeyMod::Ctrl] modifier is present.
    ///
    /// *C style function: [ncinput_ctrl_p()][c_api::ncinput_ctrl_p].*
    pub const fn ctrl_p(&self) -> bool {
        c_api::ncinput_ctrl_p(self)
    }

    /// Returns true if the [`Meta`][crate::NcKeyMod::Meta] modifier is present.
    ///
    /// *C style function: [ncinput_meta_p()][c_api::ncinput_meta_p].*
    pub const fn meta_p(&self) -> bool {
        c_api::ncinput_meta_p(self)
    }

    /// Returns true if the [`Super`][crate::NcKeyMod::Super] modifier is present.
    ///
    /// *C style function: [ncinput_super_p()][c_api::ncinput_super_p].*
    pub const fn super_p(&self) -> bool {
        c_api::ncinput_super_p(self)
    }

    /// Returns true if the [`Hyper`][crate::NcKeyMod::Hyper] modifier is present.
    ///
    /// *C style function: [ncinput_hyper_p()][c_api::ncinput_hyper_p].*
    pub const fn hyper_p(&self) -> bool {
        c_api::ncinput_hyper_p(self)
    }

    /// Returns true if the [`CapsLock`][crate::NcKeyMod::CapsLock] modifier is present.
    ///
    /// *C style function: [ncinput_capslock_p()][c_api::ncinput_capslock_p].*
    pub const fn capslock_p(&self) -> bool {
        c_api::ncinput_capslock_p(self)
    }

    /// Returns true if the [`NumLock`][crate::NcKeyMod::NumLock] modifier is present.
    ///
    /// *C style function: [ncinput_numlock_p()][c_api::ncinput_numlock_p].*
    pub const fn numlock_p(&self) -> bool {
        c_api::ncinput_numlock_p(self)
    }

    /// Returns true if both `NcInput`s are equal.
    ///
    /// *C style function: [ncinput_equal_p()][c_api::ncinput_equal_p].*
    pub const fn equal_p(&self, other: &NcInput) -> bool {
        c_api::ncinput_equal_p(self, other)
    }
}

pub(crate) mod constants {
    use crate::NcMiceEvents;

    /// *Unknown* input type event.
    pub const NCINTYPE_UNKNOWN: u32 = crate::bindings::ffi::ncintype_e_NCTYPE_UNKNOWN;

    /// *Press* input type event.
    pub const NCINTYPE_PRESS: u32 = crate::bindings::ffi::ncintype_e_NCTYPE_PRESS;

    /// *Repeat* input type event.
    pub const NCINTYPE_REPEAT: u32 = crate::bindings::ffi::ncintype_e_NCTYPE_REPEAT;

    /// *Release* input type event.
    pub const NCINTYPE_RELEASE: u32 = crate::bindings::ffi::ncintype_e_NCTYPE_RELEASE;

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
