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

use core::ffi::CStr;

use crate::NcKeyMod;

pub(crate) mod reimplemented;

mod input_type;
pub use input_type::NcInputType;
mod mice_events;
pub use mice_events::NcMiceEvents;
mod received;
pub use received::NcReceived;

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
pub type NcInput = crate::c_api::ffi::ncinput;

mod std_impls {
    use super::NcInput;

    impl PartialEq for NcInput {
        fn eq(&self, other: &Self) -> bool {
            self.equal_p(other)
        }
    }
}

/// # Constructors
impl NcInput {
    /// New empty `NcInput`.
    pub fn new_empty() -> NcInput {
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
    pub fn new(id: char) -> NcInput {
        Self::with_all_args(id, None, None, NcKeyMod::None, NcInputType::Unknown)
    }

    /// New `NcInput` with `alt` key.
    pub fn with_alt(id: char) -> NcInput {
        Self::with_all_args(id, None, None, NcKeyMod::Alt, NcInputType::Unknown)
    }

    /// New `NcInput` with `shift` key.
    pub fn with_shift(id: char) -> NcInput {
        Self::with_all_args(id, None, None, NcKeyMod::Shift, NcInputType::Unknown)
    }

    /// New `NcInput` with `ctrl` key.
    pub fn with_ctrl(id: char) -> NcInput {
        Self::with_all_args(id, None, None, NcKeyMod::Ctrl, NcInputType::Unknown)
    }

    /// New `NcInput`, expecting all the arguments (except utf8).
    pub fn with_all_args(
        id: char,
        x: Option<u32>,
        y: Option<u32>,
        modifiers: NcKeyMod,
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
            alt: false,
            shift: false,
            ctrl: false,
            evtype: evtype as u32,
            ypx: -1,
            xpx: -1,
            modifiers: modifiers.into(),
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
    /// *C style function: [ncinput_nomod_p()][crate::c_api::ncinput_nomod_p].*
    pub fn nomod_p(&self) -> bool {
        crate::c_api::ncinput_nomod_p(self)
    }

    /// Returns true if the [`Shift`][crate::NcKeyMod::Shift] modifier is present.
    ///
    /// *C style function: [ncinput_shift_p()][crate::c_api::ncinput_shift_p].*
    pub fn shift_p(&self) -> bool {
        crate::c_api::ncinput_shift_p(self)
    }

    /// Returns true if the [`Alt`][crate::NcKeyMod::Alt] modifier is present.
    ///
    /// *C style function: [ncinput_alt_p()][crate::c_api::ncinput_alt_p].*
    pub fn alt_p(&self) -> bool {
        crate::c_api::ncinput_alt_p(self)
    }

    /// Returns true if the [`Ctrl`][crate::NcKeyMod::Ctrl] modifier is present.
    ///
    /// *C style function: [ncinput_ctrl_p()][crate::c_api::ncinput_ctrl_p].*
    pub fn ctrl_p(&self) -> bool {
        crate::c_api::ncinput_ctrl_p(self)
    }

    /// Returns true if the [`Meta`][crate::NcKeyMod::Meta] modifier is present.
    ///
    /// *C style function: [ncinput_meta_p()][crate::c_api::ncinput_meta_p].*
    pub fn meta_p(&self) -> bool {
        crate::c_api::ncinput_meta_p(self)
    }

    /// Returns true if the [`Super`][crate::NcKeyMod::Super] modifier is present.
    ///
    /// *C style function: [ncinput_super_p()][crate::c_api::ncinput_super_p].*
    pub fn super_p(&self) -> bool {
        crate::c_api::ncinput_super_p(self)
    }

    /// Returns true if the [`Hyper`][crate::NcKeyMod::Hyper] modifier is present.
    ///
    /// *C style function: [ncinput_hyper_p()][crate::c_api::ncinput_hyper_p].*
    pub fn hyper_p(&self) -> bool {
        crate::c_api::ncinput_hyper_p(self)
    }

    /// Returns true if the [`CapsLock`][crate::NcKeyMod::CapsLock] modifier is present.
    ///
    /// *C style function: [ncinput_capslock_p()][crate::c_api::ncinput_capslock_p].*
    pub fn capslock_p(&self) -> bool {
        crate::c_api::ncinput_capslock_p(self)
    }

    /// Returns true if the [`NumLock`][crate::NcKeyMod::NumLock] modifier is present.
    ///
    /// *C style function: [ncinput_numlock_p()][crate::c_api::ncinput_numlock_p].*
    pub fn numlock_p(&self) -> bool {
        crate::c_api::ncinput_numlock_p(self)
    }

    /// Returns true if both `NcInput`s are equal.
    ///
    /// *C style function: [ncinput_equal_p()][crate::c_api::ncinput_equal_p].*
    pub fn equal_p(&self, other: &NcInput) -> bool {
        crate::c_api::ncinput_equal_p(self, other)
    }
}

pub(crate) mod c_api {
    pub use super::input_type::c_api::*;
    pub use super::mice_events::c_api::*;
}
