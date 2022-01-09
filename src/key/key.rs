//!

use super::constants;
use crate::c_api;

/// Synthesized input events (any input event we can report that isn't
/// representative of some Unicode).
///
/// This covers both keyboard and mouse events, as well as signals and even
/// window events.
///
/// Rather than using one of the Private Use Areas of Unicode, we use the area
/// beyond the 17 65536-entry Planes (1114112).
///
/// We round up to 5000 so that it's trivial to identify synthesized characters.
/// based on their numeric definition here.
///
/// This is safe, since we needn't convert these synthesized characters
/// into UTF8 (they would otherwise require more than four bytes).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NcKey(pub u32);

impl From<NcKey> for u32 {
    fn from(k: NcKey) -> Self {
        k.0
    }
}

/// # Methods
impl NcKey {
    /// Checks whether a number falls in the range of synthesized events.
    pub fn is(num: u32) -> bool {
        c_api::nckey_synthesized_p(num) || num == NcKey::ESC.0 || num == NcKey::TAB.0
    }

    /// Returns a new `NcKey` if the provided number falls in the correct range.
    pub fn new(num: u32) -> Option<Self> {
        if Self::is(num) {
            Some(Self(num))
        } else {
            None
        }
    }

    /// Returns the name of the current `NcKey`.
    pub fn name(&self) -> &'static str {
        Self::check_name(self.0)
    }

    /// Returns the name of the `NcKey` the number would be.
    pub fn check_name(num: u32) -> &'static str {
        if Self::is(num) {
            match Self(num) {
                Self::INVALID => "INVALID",
                Self::RESIZE => "RESIZE",
                Self::UP => "UP",
                Self::RIGHT => "RIGHT",
                Self::DOWN => "DOWN",
                Self::LEFT => "LEFT",
                Self::INS => "INS",
                Self::DEL => "DEL",
                Self::BACKSPACE => "BACKSPACE",
                Self::PGDOWN => "PGDOWN",
                Self::PGUP => "PGUP",
                Self::HOME => "HOME",
                Self::END => "END",
                Self::F00 => "F00",
                Self::F01 => "F01",
                Self::F02 => "F02",
                Self::F03 => "F03",
                Self::F04 => "F04",
                Self::F05 => "F05",
                Self::F06 => "F06",
                Self::F07 => "F07",
                Self::F08 => "F08",
                Self::F09 => "F09",
                Self::F10 => "F10",
                Self::F11 => "F11",
                Self::F12 => "F12",
                Self::F13 => "F13",
                Self::F14 => "F14",
                Self::F15 => "F15",
                Self::F16 => "F16",
                Self::F17 => "F17",
                Self::F18 => "F18",
                Self::F19 => "F19",
                Self::F20 => "F20",
                Self::F21 => "F21",
                Self::F22 => "F22",
                Self::F23 => "F23",
                Self::F24 => "F24",
                Self::F25 => "F25",
                Self::F26 => "F26",
                Self::F27 => "F27",
                Self::F28 => "F28",
                Self::F29 => "F29",
                Self::F30 => "F30",
                Self::F31 => "F31",
                Self::F32 => "F32",
                Self::F33 => "F33",
                Self::F34 => "F34",
                Self::F35 => "F35",
                Self::F36 => "F36",
                Self::F37 => "F37",
                Self::F38 => "F38",
                Self::F39 => "F39",
                Self::F40 => "F40",
                Self::F41 => "F41",
                Self::F42 => "F42",
                Self::F43 => "F43",
                Self::F44 => "F44",
                Self::F45 => "F45",
                Self::F46 => "F46",
                Self::F47 => "F47",
                Self::F48 => "F48",
                Self::F49 => "F49",
                Self::F50 => "F50",
                Self::F51 => "F51",
                Self::F52 => "F52",
                Self::F53 => "F53",
                Self::F54 => "F54",
                Self::F55 => "F55",
                Self::F56 => "F56",
                Self::F57 => "F57",
                Self::F58 => "F58",
                Self::F59 => "F59",
                Self::F60 => "F60",

                Self::ENTER => "ENTER",
                Self::CLS => "CLS",
                Self::DLEFT => "DLEFT",
                Self::DRIGHT => "DRIGHT",
                Self::ULEFT => "ULEFT",
                Self::URIGHT => "URIGHT",
                Self::CENTER => "CENTER",
                Self::BEGIN => "BEGIN",
                Self::CANCEL => "CANCEL",
                Self::CLOSE => "CLOSE",
                Self::COMMAND => "COMMAND",
                Self::COPY => "COPY",
                Self::EXIT => "EXIT",
                Self::PRINT => "PRINT",
                Self::REFRESH => "REFRESH",

                Self::CAPS_LOCK => "CAPS_LOCK",
                Self::SCROLL_LOCK => "SCROLL_LOCK",
                Self::NUM_LOCK => "NUM_LOCK",
                Self::PRINT_SCREEN => "PRINT_SCREEN",
                Self::PAUSE => "PAUSE",
                Self::MENU => "MENU",

                Self::MEDIA_PLAY => "MEDIA_PLAY",
                Self::MEDIA_PAUSE => "MEDIA_PAUSE",
                Self::MEDIA_PPAUSE => "MEDIA_PPAUSE",
                Self::MEDIA_REV => "MEDIA_REV",
                Self::MEDIA_STOP => "MEDIA_STOP",
                Self::MEDIA_FF => "MEDIA_FF",
                Self::MEDIA_REWIND => "MEDIA_REWIND",
                Self::MEDIA_NEXT => "MEDIA_NEXT",
                Self::MEDIA_PREV => "MEDIA_PREV",
                Self::MEDIA_RECORD => "MEDIA_RECORD",
                Self::MEDIA_LVOL => "MEDIA_LVOL",
                Self::MEDIA_RVOL => "MEDIA_RVOL",
                Self::MEDIA_MUTE => "MEDIA_MUTE",

                Self::LSHIFT => "LSHIFT",
                Self::LCTRL => "LCTRL",
                Self::LALT => "LALT",
                Self::LSUPER => "LSUPER",
                Self::LHYPER => "LHYPER",
                Self::LMETA => "LMETA",
                Self::RSHIFT => "RSHIFT",
                Self::RCTRL => "RCTRL",
                Self::RALT => "RALT",
                Self::RSUPER => "RSUPER",
                Self::RHYPER => "RHYPER",
                Self::RMETA => "RMETA",

                Self::MOTION => "MOTION",
                Self::BUTTON1 => "BUTTON1",
                Self::BUTTON2 => "BUTTON2",
                Self::BUTTON3 => "BUTTON3",
                // SCROLL_UP
                Self::BUTTON4 => "BUTTON4",
                // SCROLL_DOWN
                Self::BUTTON5 => "BUTTON5",
                Self::BUTTON6 => "BUTTON6",
                Self::BUTTON7 => "BUTTON7",
                Self::BUTTON8 => "BUTTON8",
                Self::BUTTON9 => "BUTTON9",
                Self::BUTTON10 => "BUTTON10",
                Self::BUTTON11 => "BUTTON11",
                Self::EOF => "EOF",

                Self::TAB => "TAB",
                Self::ESC => "ESC",
                Self::SPACE => "SPACE",
                _ => "",
            }
        } else {
            ""
        }
    }
}

/// # Constants
impl NcKey {
    /// Rather than using one of the Private Use Areas of Unicode, we use the
    /// area beyond the 17 65536-entry Planes (1114112).
    ///
    /// We round up to 5000 so that it's trivial to identify synthesized
    /// characters based on their numeric definition here.
    ///
    /// This is safe, since we needn't convert these synthesized characters
    /// into UTF8 (they would otherwise require more than four bytes).
    pub const PRETERUNICODEBASE: u32 = 1115000;

    pub const INVALID: NcKey = NcKey(constants::NCKEY_INVALID);
    /// we received SIGWINCH
    pub const RESIZE: NcKey = NcKey(constants::NCKEY_RESIZE);
    pub const UP: NcKey = NcKey(constants::NCKEY_UP);
    pub const RIGHT: NcKey = NcKey(constants::NCKEY_RIGHT);
    pub const DOWN: NcKey = NcKey(constants::NCKEY_DOWN);
    pub const LEFT: NcKey = NcKey(constants::NCKEY_LEFT);
    pub const INS: NcKey = NcKey(constants::NCKEY_INS);
    pub const DEL: NcKey = NcKey(constants::NCKEY_DEL);
    /// backspace (sometimes)
    pub const BACKSPACE: NcKey = NcKey(constants::NCKEY_BACKSPACE);
    pub const PGDOWN: NcKey = NcKey(constants::NCKEY_PGDOWN);
    pub const PGUP: NcKey = NcKey(constants::NCKEY_PGUP);
    pub const HOME: NcKey = NcKey(constants::NCKEY_HOME);
    pub const END: NcKey = NcKey(constants::NCKEY_END);
    pub const F00: NcKey = NcKey(constants::NCKEY_F00);
    pub const F01: NcKey = NcKey(constants::NCKEY_F01);
    pub const F02: NcKey = NcKey(constants::NCKEY_F02);
    pub const F03: NcKey = NcKey(constants::NCKEY_F03);
    pub const F04: NcKey = NcKey(constants::NCKEY_F04);
    pub const F05: NcKey = NcKey(constants::NCKEY_F05);
    pub const F06: NcKey = NcKey(constants::NCKEY_F06);
    pub const F07: NcKey = NcKey(constants::NCKEY_F07);
    pub const F08: NcKey = NcKey(constants::NCKEY_F08);
    pub const F09: NcKey = NcKey(constants::NCKEY_F09);
    pub const F10: NcKey = NcKey(constants::NCKEY_F10);
    pub const F11: NcKey = NcKey(constants::NCKEY_F11);
    pub const F12: NcKey = NcKey(constants::NCKEY_F12);
    pub const F13: NcKey = NcKey(constants::NCKEY_F13);
    pub const F14: NcKey = NcKey(constants::NCKEY_F14);
    pub const F15: NcKey = NcKey(constants::NCKEY_F15);
    pub const F16: NcKey = NcKey(constants::NCKEY_F16);
    pub const F17: NcKey = NcKey(constants::NCKEY_F17);
    pub const F18: NcKey = NcKey(constants::NCKEY_F18);
    pub const F19: NcKey = NcKey(constants::NCKEY_F19);
    pub const F20: NcKey = NcKey(constants::NCKEY_F20);
    pub const F21: NcKey = NcKey(constants::NCKEY_F21);
    pub const F22: NcKey = NcKey(constants::NCKEY_F22);
    pub const F23: NcKey = NcKey(constants::NCKEY_F23);
    pub const F24: NcKey = NcKey(constants::NCKEY_F24);
    pub const F25: NcKey = NcKey(constants::NCKEY_F25);
    pub const F26: NcKey = NcKey(constants::NCKEY_F26);
    pub const F27: NcKey = NcKey(constants::NCKEY_F27);
    pub const F28: NcKey = NcKey(constants::NCKEY_F28);
    pub const F29: NcKey = NcKey(constants::NCKEY_F29);
    pub const F30: NcKey = NcKey(constants::NCKEY_F30);
    pub const F31: NcKey = NcKey(constants::NCKEY_F31);
    pub const F32: NcKey = NcKey(constants::NCKEY_F32);
    pub const F33: NcKey = NcKey(constants::NCKEY_F33);
    pub const F34: NcKey = NcKey(constants::NCKEY_F34);
    pub const F35: NcKey = NcKey(constants::NCKEY_F35);
    pub const F36: NcKey = NcKey(constants::NCKEY_F36);
    pub const F37: NcKey = NcKey(constants::NCKEY_F37);
    pub const F38: NcKey = NcKey(constants::NCKEY_F38);
    pub const F39: NcKey = NcKey(constants::NCKEY_F39);
    pub const F40: NcKey = NcKey(constants::NCKEY_F40);
    pub const F41: NcKey = NcKey(constants::NCKEY_F41);
    pub const F42: NcKey = NcKey(constants::NCKEY_F42);
    pub const F43: NcKey = NcKey(constants::NCKEY_F43);
    pub const F44: NcKey = NcKey(constants::NCKEY_F44);
    pub const F45: NcKey = NcKey(constants::NCKEY_F45);
    pub const F46: NcKey = NcKey(constants::NCKEY_F46);
    pub const F47: NcKey = NcKey(constants::NCKEY_F47);
    pub const F48: NcKey = NcKey(constants::NCKEY_F48);
    pub const F49: NcKey = NcKey(constants::NCKEY_F49);
    pub const F50: NcKey = NcKey(constants::NCKEY_F50);
    pub const F51: NcKey = NcKey(constants::NCKEY_F51);
    pub const F52: NcKey = NcKey(constants::NCKEY_F52);
    pub const F53: NcKey = NcKey(constants::NCKEY_F53);
    pub const F54: NcKey = NcKey(constants::NCKEY_F54);
    pub const F55: NcKey = NcKey(constants::NCKEY_F55);
    pub const F56: NcKey = NcKey(constants::NCKEY_F56);
    pub const F57: NcKey = NcKey(constants::NCKEY_F57);
    pub const F58: NcKey = NcKey(constants::NCKEY_F58);
    pub const F59: NcKey = NcKey(constants::NCKEY_F59);
    pub const F60: NcKey = NcKey(constants::NCKEY_F60);

    // ... leave room for function keys.

    pub const ENTER: NcKey = NcKey(constants::NCKEY_ENTER);
    /// "clear-screen or erase"
    pub const CLS: NcKey = NcKey(constants::NCKEY_CLS);
    /// down + left on keypad
    pub const DLEFT: NcKey = NcKey(constants::NCKEY_DLEFT);
    pub const DRIGHT: NcKey = NcKey(constants::NCKEY_DRIGHT);
    /// up + left on keypad
    pub const ULEFT: NcKey = NcKey(constants::NCKEY_ULEFT);
    pub const URIGHT: NcKey = NcKey(constants::NCKEY_URIGHT);
    /// the most truly neutral of keypresses
    pub const CENTER: NcKey = NcKey(constants::NCKEY_CENTER);
    pub const BEGIN: NcKey = NcKey(constants::NCKEY_BEGIN);
    pub const CANCEL: NcKey = NcKey(constants::NCKEY_CANCEL);
    pub const CLOSE: NcKey = NcKey(constants::NCKEY_CLOSE);
    pub const COMMAND: NcKey = NcKey(constants::NCKEY_COMMAND);
    pub const COPY: NcKey = NcKey(constants::NCKEY_COPY);
    pub const EXIT: NcKey = NcKey(constants::NCKEY_EXIT);
    pub const PRINT: NcKey = NcKey(constants::NCKEY_PRINT);
    pub const REFRESH: NcKey = NcKey(constants::NCKEY_REFRESH);

    // these keys aren't generally available outside of the kitty protocol:

    pub const CAPS_LOCK: NcKey = NcKey(constants::NCKEY_CAPS_LOCK);
    pub const SCROLL_LOCK: NcKey = NcKey(constants::NCKEY_SCROLL_LOCK);
    pub const NUM_LOCK: NcKey = NcKey(constants::NCKEY_NUM_LOCK);
    pub const PRINT_SCREEN: NcKey = NcKey(constants::NCKEY_PRINT_SCREEN);
    pub const PAUSE: NcKey = NcKey(constants::NCKEY_PAUSE);
    pub const MENU: NcKey = NcKey(constants::NCKEY_MENU);

    // media keys, similarly only available through kitty's protocol:

    pub const MEDIA_PLAY: NcKey = NcKey(constants::NCKEY_MEDIA_PLAY);
    pub const MEDIA_PAUSE: NcKey = NcKey(constants::NCKEY_MEDIA_PAUSE);
    pub const MEDIA_PPAUSE: NcKey = NcKey(constants::NCKEY_MEDIA_PPAUSE);
    pub const MEDIA_REV: NcKey = NcKey(constants::NCKEY_MEDIA_REV);
    pub const MEDIA_STOP: NcKey = NcKey(constants::NCKEY_MEDIA_STOP);
    pub const MEDIA_FF: NcKey = NcKey(constants::NCKEY_MEDIA_FF);
    pub const MEDIA_REWIND: NcKey = NcKey(constants::NCKEY_MEDIA_REWIND);
    pub const MEDIA_NEXT: NcKey = NcKey(constants::NCKEY_MEDIA_NEXT);
    pub const MEDIA_PREV: NcKey = NcKey(constants::NCKEY_MEDIA_PREV);
    pub const MEDIA_RECORD: NcKey = NcKey(constants::NCKEY_MEDIA_RECORD);
    pub const MEDIA_LVOL: NcKey = NcKey(constants::NCKEY_MEDIA_LVOL);
    pub const MEDIA_RVOL: NcKey = NcKey(constants::NCKEY_MEDIA_RVOL);
    pub const MEDIA_MUTE: NcKey = NcKey(constants::NCKEY_MEDIA_MUTE);

    // modifiers when pressed by themselves. this ordering comes from the Kitty
    // keyboard protocol, and mustn't be changed without updating handlers:

    pub const LSHIFT: NcKey = NcKey(constants::NCKEY_LSHIFT);
    pub const LCTRL: NcKey = NcKey(constants::NCKEY_LCTRL);
    pub const LALT: NcKey = NcKey(constants::NCKEY_LALT);
    pub const LSUPER: NcKey = NcKey(constants::NCKEY_LSUPER);
    pub const LHYPER: NcKey = NcKey(constants::NCKEY_LHYPER);
    pub const LMETA: NcKey = NcKey(constants::NCKEY_LMETA);
    pub const RSHIFT: NcKey = NcKey(constants::NCKEY_RSHIFT);
    pub const RCTRL: NcKey = NcKey(constants::NCKEY_RCTRL);
    pub const RALT: NcKey = NcKey(constants::NCKEY_RALT);
    pub const RSUPER: NcKey = NcKey(constants::NCKEY_RSUPER);
    pub const RHYPER: NcKey = NcKey(constants::NCKEY_RHYPER);
    pub const RMETA: NcKey = NcKey(constants::NCKEY_RMETA);

    // Mouse events. We encode which button was pressed into the number,
    // but position information is embedded in the larger ncinput event:

    pub const MOTION: NcKey = NcKey(constants::NCKEY_MOTION);
    pub const BUTTON1: NcKey = NcKey(constants::NCKEY_BUTTON1);
    pub const BUTTON2: NcKey = NcKey(constants::NCKEY_BUTTON2);
    pub const BUTTON3: NcKey = NcKey(constants::NCKEY_BUTTON3);
    /// scrollwheel up
    pub const BUTTON4: NcKey = NcKey(constants::NCKEY_BUTTON4);
    /// scrollwheel down
    pub const BUTTON5: NcKey = NcKey(constants::NCKEY_BUTTON5);
    pub const BUTTON6: NcKey = NcKey(constants::NCKEY_BUTTON6);
    pub const BUTTON7: NcKey = NcKey(constants::NCKEY_BUTTON7);
    pub const BUTTON8: NcKey = NcKey(constants::NCKEY_BUTTON8);
    pub const BUTTON9: NcKey = NcKey(constants::NCKEY_BUTTON9);
    pub const BUTTON10: NcKey = NcKey(constants::NCKEY_BUTTON10);
    pub const BUTTON11: NcKey = NcKey(constants::NCKEY_BUTTON11);

    /// we received SIGCONT
    pub const SIGNAL: NcKey = NcKey(constants::NCKEY_SIGNAL);

    /// Will be returned upon reaching the end of input.
    pub const EOF: NcKey = NcKey(constants::NCKEY_EOF);

    // Synonyms (so far as we're concerned):

    pub const SCROLL_UP: NcKey = NcKey(constants::NCKEY_SCROLL_UP);
    pub const SCROLL_DOWN: NcKey = NcKey(constants::NCKEY_SCROLL_DOWN);
    pub const RETURN: NcKey = NcKey(constants::NCKEY_RETURN);

    // Aliases, from the 128 characters common to ASCII+UTF8:

    pub const TAB: NcKey = NcKey(constants::NCKEY_TAB);
    pub const ESC: NcKey = NcKey(constants::NCKEY_ESC);
    pub const SPACE: NcKey = NcKey(constants::NCKEY_SPACE);
}
