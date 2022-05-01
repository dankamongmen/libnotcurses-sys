//!

/// A synthesized input event.
///
/// This means any input event we can report that isn't representative
/// of some Unicode. This covers both keyboard and mouse events, as well as
/// signals and even window events.
///
/// See also [`PRETERUNICODEBASE`][c_api::PRETERUNICODEBASE].
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct NcKey(pub u32);

mod std_impls {
    use super::NcKey;
    use std::fmt;

    impl fmt::Display for NcKey {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.name())
        }
    }

    impl fmt::Debug for NcKey {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Key::{}", self)
        }
    }

    impl From<NcKey> for u32 {
        fn from(k: NcKey) -> Self {
            k.0
        }
    }
}

/// # Constants
impl NcKey {
    pub const Invalid: NcKey = NcKey(c_api::NCKEY_INVALID);
    /// we received `SIGWINCH`.
    pub const Resize: NcKey = NcKey(c_api::NCKEY_RESIZE);
    pub const Up: NcKey = NcKey(c_api::NCKEY_UP);
    pub const Right: NcKey = NcKey(c_api::NCKEY_RIGHT);
    pub const Down: NcKey = NcKey(c_api::NCKEY_DOWN);
    pub const Left: NcKey = NcKey(c_api::NCKEY_LEFT);
    pub const Ins: NcKey = NcKey(c_api::NCKEY_INS);
    pub const Del: NcKey = NcKey(c_api::NCKEY_DEL);
    pub const Backspace: NcKey = NcKey(c_api::NCKEY_BACKSPACE);
    pub const PgDown: NcKey = NcKey(c_api::NCKEY_PGDOWN);
    pub const PgUp: NcKey = NcKey(c_api::NCKEY_PGUP);
    pub const Home: NcKey = NcKey(c_api::NCKEY_HOME);
    pub const End: NcKey = NcKey(c_api::NCKEY_END);
    pub const F00: NcKey = NcKey(c_api::NCKEY_F00);
    pub const F01: NcKey = NcKey(c_api::NCKEY_F01);
    pub const F02: NcKey = NcKey(c_api::NCKEY_F02);
    pub const F03: NcKey = NcKey(c_api::NCKEY_F03);
    pub const F04: NcKey = NcKey(c_api::NCKEY_F04);
    pub const F05: NcKey = NcKey(c_api::NCKEY_F05);
    pub const F06: NcKey = NcKey(c_api::NCKEY_F06);
    pub const F07: NcKey = NcKey(c_api::NCKEY_F07);
    pub const F08: NcKey = NcKey(c_api::NCKEY_F08);
    pub const F09: NcKey = NcKey(c_api::NCKEY_F09);
    pub const F10: NcKey = NcKey(c_api::NCKEY_F10);
    pub const F11: NcKey = NcKey(c_api::NCKEY_F11);
    pub const F12: NcKey = NcKey(c_api::NCKEY_F12);
    pub const F13: NcKey = NcKey(c_api::NCKEY_F13);
    pub const F14: NcKey = NcKey(c_api::NCKEY_F14);
    pub const F15: NcKey = NcKey(c_api::NCKEY_F15);
    pub const F16: NcKey = NcKey(c_api::NCKEY_F16);
    pub const F17: NcKey = NcKey(c_api::NCKEY_F17);
    pub const F18: NcKey = NcKey(c_api::NCKEY_F18);
    pub const F19: NcKey = NcKey(c_api::NCKEY_F19);
    pub const F20: NcKey = NcKey(c_api::NCKEY_F20);
    pub const F21: NcKey = NcKey(c_api::NCKEY_F21);
    pub const F22: NcKey = NcKey(c_api::NCKEY_F22);
    pub const F23: NcKey = NcKey(c_api::NCKEY_F23);
    pub const F24: NcKey = NcKey(c_api::NCKEY_F24);
    pub const F25: NcKey = NcKey(c_api::NCKEY_F25);
    pub const F26: NcKey = NcKey(c_api::NCKEY_F26);
    pub const F27: NcKey = NcKey(c_api::NCKEY_F27);
    pub const F28: NcKey = NcKey(c_api::NCKEY_F28);
    pub const F29: NcKey = NcKey(c_api::NCKEY_F29);
    pub const F30: NcKey = NcKey(c_api::NCKEY_F30);
    pub const F31: NcKey = NcKey(c_api::NCKEY_F31);
    pub const F32: NcKey = NcKey(c_api::NCKEY_F32);
    pub const F33: NcKey = NcKey(c_api::NCKEY_F33);
    pub const F34: NcKey = NcKey(c_api::NCKEY_F34);
    pub const F35: NcKey = NcKey(c_api::NCKEY_F35);
    pub const F36: NcKey = NcKey(c_api::NCKEY_F36);
    pub const F37: NcKey = NcKey(c_api::NCKEY_F37);
    pub const F38: NcKey = NcKey(c_api::NCKEY_F38);
    pub const F39: NcKey = NcKey(c_api::NCKEY_F39);
    pub const F40: NcKey = NcKey(c_api::NCKEY_F40);
    pub const F41: NcKey = NcKey(c_api::NCKEY_F41);
    pub const F42: NcKey = NcKey(c_api::NCKEY_F42);
    pub const F43: NcKey = NcKey(c_api::NCKEY_F43);
    pub const F44: NcKey = NcKey(c_api::NCKEY_F44);
    pub const F45: NcKey = NcKey(c_api::NCKEY_F45);
    pub const F46: NcKey = NcKey(c_api::NCKEY_F46);
    pub const F47: NcKey = NcKey(c_api::NCKEY_F47);
    pub const F48: NcKey = NcKey(c_api::NCKEY_F48);
    pub const F49: NcKey = NcKey(c_api::NCKEY_F49);
    pub const F50: NcKey = NcKey(c_api::NCKEY_F50);
    pub const F51: NcKey = NcKey(c_api::NCKEY_F51);
    pub const F52: NcKey = NcKey(c_api::NCKEY_F52);
    pub const F53: NcKey = NcKey(c_api::NCKEY_F53);
    pub const F54: NcKey = NcKey(c_api::NCKEY_F54);
    pub const F55: NcKey = NcKey(c_api::NCKEY_F55);
    pub const F56: NcKey = NcKey(c_api::NCKEY_F56);
    pub const F57: NcKey = NcKey(c_api::NCKEY_F57);
    pub const F58: NcKey = NcKey(c_api::NCKEY_F58);
    pub const F59: NcKey = NcKey(c_api::NCKEY_F59);
    pub const F60: NcKey = NcKey(c_api::NCKEY_F60);

    // ... leave room for function keys.

    pub const Enter: NcKey = NcKey(c_api::NCKEY_ENTER);
    /// "clear-screen or erase"
    pub const Cls: NcKey = NcKey(c_api::NCKEY_CLS);
    /// down + left on keypad
    pub const DLeft: NcKey = NcKey(c_api::NCKEY_DLEFT);
    pub const DRight: NcKey = NcKey(c_api::NCKEY_DRIGHT);
    /// up + left on keypad
    pub const ULeft: NcKey = NcKey(c_api::NCKEY_ULEFT);
    pub const URight: NcKey = NcKey(c_api::NCKEY_URIGHT);
    pub const Center: NcKey = NcKey(c_api::NCKEY_CENTER);
    pub const Begin: NcKey = NcKey(c_api::NCKEY_BEGIN);
    pub const Cancel: NcKey = NcKey(c_api::NCKEY_CANCEL);
    pub const Close: NcKey = NcKey(c_api::NCKEY_CLOSE);
    pub const Command: NcKey = NcKey(c_api::NCKEY_COMMAND);
    pub const Copy: NcKey = NcKey(c_api::NCKEY_COPY);
    pub const Exit: NcKey = NcKey(c_api::NCKEY_EXIT);
    pub const Print: NcKey = NcKey(c_api::NCKEY_PRINT);
    pub const Refresh: NcKey = NcKey(c_api::NCKEY_REFRESH);

    // these keys aren't generally available outside of the kitty protocol:

    pub const CapsLock: NcKey = NcKey(c_api::NCKEY_CAPS_LOCK);
    pub const ScrollLock: NcKey = NcKey(c_api::NCKEY_SCROLL_LOCK);
    pub const NumLock: NcKey = NcKey(c_api::NCKEY_NUM_LOCK);
    pub const PrintScreen: NcKey = NcKey(c_api::NCKEY_PRINT_SCREEN);
    pub const Pause: NcKey = NcKey(c_api::NCKEY_PAUSE);
    pub const Menu: NcKey = NcKey(c_api::NCKEY_MENU);

    // media keys, similarly only available through kitty's protocol:

    pub const MediaPlay: NcKey = NcKey(c_api::NCKEY_MEDIA_PLAY);
    pub const MediaPause: NcKey = NcKey(c_api::NCKEY_MEDIA_PAUSE);
    pub const MediaPPause: NcKey = NcKey(c_api::NCKEY_MEDIA_PPAUSE);
    pub const MediaRev: NcKey = NcKey(c_api::NCKEY_MEDIA_REV);
    pub const MediaStop: NcKey = NcKey(c_api::NCKEY_MEDIA_STOP);
    pub const MediaFF: NcKey = NcKey(c_api::NCKEY_MEDIA_FF);
    pub const MediaRewind: NcKey = NcKey(c_api::NCKEY_MEDIA_REWIND);
    pub const MediaNext: NcKey = NcKey(c_api::NCKEY_MEDIA_NEXT);
    pub const MediaPrev: NcKey = NcKey(c_api::NCKEY_MEDIA_PREV);
    pub const MediaRecord: NcKey = NcKey(c_api::NCKEY_MEDIA_RECORD);
    pub const MediaLVol: NcKey = NcKey(c_api::NCKEY_MEDIA_LVOL);
    pub const MediaRVol: NcKey = NcKey(c_api::NCKEY_MEDIA_RVOL);
    pub const MediaMute: NcKey = NcKey(c_api::NCKEY_MEDIA_MUTE);

    // modifiers when pressed by themselves. this ordering comes from the Kitty
    // keyboard protocol, and mustn't be changed without updating handlers:

    pub const LShift: NcKey = NcKey(c_api::NCKEY_LSHIFT);
    pub const LCtrl: NcKey = NcKey(c_api::NCKEY_LCTRL);
    pub const LAlt: NcKey = NcKey(c_api::NCKEY_LALT);
    pub const LSuper: NcKey = NcKey(c_api::NCKEY_LSUPER);
    pub const LHyper: NcKey = NcKey(c_api::NCKEY_LHYPER);
    pub const LMeta: NcKey = NcKey(c_api::NCKEY_LMETA);
    pub const RShift: NcKey = NcKey(c_api::NCKEY_RSHIFT);
    pub const RCtrl: NcKey = NcKey(c_api::NCKEY_RCTRL);
    pub const RAlt: NcKey = NcKey(c_api::NCKEY_RALT);
    pub const RSuper: NcKey = NcKey(c_api::NCKEY_RSUPER);
    pub const RHyper: NcKey = NcKey(c_api::NCKEY_RHYPER);
    pub const RMeta: NcKey = NcKey(c_api::NCKEY_RMETA);
    /// `AltGr` in european keyboards
    pub const L3Shift: NcKey = NcKey(c_api::NCKEY_L3SHIFT);
    pub const L5Shift: NcKey = NcKey(c_api::NCKEY_L5SHIFT);

    // Mouse events. We encode which button was pressed into the number,
    // but position information is embedded in the larger ncinput event:

    pub const Motion: NcKey = NcKey(c_api::NCKEY_MOTION);
    pub const Button1: NcKey = NcKey(c_api::NCKEY_BUTTON1);
    pub const Button2: NcKey = NcKey(c_api::NCKEY_BUTTON2);
    pub const Button3: NcKey = NcKey(c_api::NCKEY_BUTTON3);
    /// scrollwheel up
    pub const Button4: NcKey = NcKey(c_api::NCKEY_BUTTON4);
    /// scrollwheel down
    pub const Button5: NcKey = NcKey(c_api::NCKEY_BUTTON5);
    pub const Button6: NcKey = NcKey(c_api::NCKEY_BUTTON6);
    pub const Button7: NcKey = NcKey(c_api::NCKEY_BUTTON7);
    pub const Button8: NcKey = NcKey(c_api::NCKEY_BUTTON8);
    pub const Button9: NcKey = NcKey(c_api::NCKEY_BUTTON9);
    pub const Button10: NcKey = NcKey(c_api::NCKEY_BUTTON10);
    pub const Button11: NcKey = NcKey(c_api::NCKEY_BUTTON11);

    /// we received SIGCONT
    pub const Signal: NcKey = NcKey(c_api::NCKEY_SIGNAL);

    /// Will be returned upon reaching the end of input.
    pub const Eof: NcKey = NcKey(c_api::NCKEY_EOF);

    // Aliases from the 128 characters common to ASCII+UTF8:
    pub const Tab: NcKey = NcKey(c_api::NCKEY_TAB);
    pub const Esc: NcKey = NcKey(c_api::NCKEY_ESC);
    pub const Space: NcKey = NcKey(c_api::NCKEY_SPACE);
}

/// # Aliases
impl NcKey {
    /// Alias of [`Button4`][NcKey::Button4]
    pub const ScrollUp: NcKey = NcKey(c_api::NCKEY_SCROLL_UP);
    /// Alias of [`Button5`][NcKey::Button5]
    pub const Scrolldown: NcKey = NcKey(c_api::NCKEY_SCROLL_DOWN);
    /// Alias of [`Enter`][NcKey::Enter]
    pub const Return: NcKey = NcKey(c_api::NCKEY_RETURN);
}

/// # Methods
impl NcKey {
    /// Checks whether a number falls in the range of synthesized events.
    pub fn is(num: u32) -> bool {
        crate::c_api::nckey_synthesized_p(num) || num == NcKey::Esc.0 || num == NcKey::Tab.0
    }

    /// Returns a new `NcKey` if the provided number falls in the correct range.
    pub fn new(num: u32) -> Option<Self> {
        if Self::is(num) {
            Some(Self(num))
        } else {
            None
        }
    }

    //

    /// Returns true if it's a function key event.
    pub fn is_function(&self) -> bool {
        match self.0 {
            c_api::NCKEY_F00..=c_api::NCKEY_F60 => true,
            _ => false,
        }
    }

    /// Returns true if it's a multimedia key event.
    pub fn is_media(&self) -> bool {
        match self.0 {
            c_api::NCKEY_MEDIA_PLAY..=c_api::NCKEY_MEDIA_MUTE => true,
            _ => false,
        }
    }

    /// Returns true if it's a mouse event.
    pub fn is_mouse(&self) -> bool {
        match self.0 {
            c_api::NCKEY_MOTION..=c_api::NCKEY_BUTTON11 => true,
            _ => false,
        }
    }

    /// Returns true if it's a resize event.
    pub fn is_resize(&self) -> bool {
        match self.0 {
            c_api::NCKEY_RESIZE => true,
            _ => false,
        }
    }

    //

    /// Returns the name of the current `NcKey`.
    pub fn name(&self) -> &'static str {
        Self::check_name(self.0)
    }

    /// Returns the name of the `NcKey` the number would be.
    pub fn check_name(num: u32) -> &'static str {
        if Self::is(num) {
            match Self(num) {
                Self::Invalid => "Invalid",
                Self::Resize => "Resize",
                Self::Up => "Up",
                Self::Right => "Right",
                Self::Down => "Down",
                Self::Left => "Left",
                Self::Ins => "Ins",
                Self::Del => "Del",
                Self::Backspace => "Backspace",
                Self::PgDown => "PgDown",
                Self::PgUp => "PgUp",
                Self::Home => "Home",
                Self::End => "End",
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

                Self::Enter => "Enter",
                Self::Cls => "Cls",
                Self::DLeft => "DLeft",
                Self::DRight => "DRight",
                Self::ULeft => "ULeft",
                Self::URight => "URight",
                Self::Center => "Center",
                Self::Begin => "Begin",
                Self::Cancel => "Cancel",
                Self::Close => "Close",
                Self::Command => "Command",
                Self::Copy => "Copy",
                Self::Exit => "Exit",
                Self::Print => "Print",
                Self::Refresh => "Refresh",

                Self::CapsLock => "CapsLock",
                Self::ScrollLock => "ScrollLock",
                Self::NumLock => "NumLock",
                Self::PrintScreen => "PrintScreen",
                Self::Pause => "Pause",
                Self::Menu => "Menu",

                Self::MediaPlay => "MediaPlay",
                Self::MediaPause => "MediaPause",
                Self::MediaPPause => "MediaPPause",
                Self::MediaRev => "MediaRev",
                Self::MediaStop => "MediaStop",
                Self::MediaFF => "MediaFF",
                Self::MediaRewind => "MediaRewind",
                Self::MediaNext => "MediaNext",
                Self::MediaPrev => "MediaPrev",
                Self::MediaRecord => "MediaRecord",
                Self::MediaLVol => "MediaLVol",
                Self::MediaRVol => "MediaRVol",
                Self::MediaMute => "MediaMute",

                Self::LShift => "LShift",
                Self::LCtrl => "LCtrl",
                Self::LAlt => "LAlt",
                Self::LSuper => "LSuper",
                Self::LHyper => "LHyper",
                Self::LMeta => "LMeta",
                Self::RShift => "RShift",
                Self::RCtrl => "RCtrl",
                Self::RAlt => "RAlt",
                Self::RSuper => "RSuper",
                Self::RHyper => "RHyper",
                Self::RMeta => "RMeta",
                Self::L3Shift => "L3Shift",
                Self::L5Shift => "L5Shift",

                Self::Motion => "Motion",
                Self::Button1 => "Button1",
                Self::Button2 => "Button2",
                Self::Button3 => "Button3",
                // scrollwheel up
                Self::Button4 => "Button4",
                // scrollwheel down
                Self::Button5 => "Button5",
                Self::Button6 => "Button6",
                Self::Button7 => "Button7",
                Self::Button8 => "Button8",
                Self::Button9 => "Button9",
                Self::Button10 => "Button10",
                Self::Button11 => "Button11",
                Self::Eof => "Eof",

                Self::Tab => "Tab",
                Self::Esc => "Esc",
                Self::Space => "Space",
                _ => "",
            }
        } else {
            ""
        }
    }
}

pub(crate) mod c_api {
    /// Offset for `NCKEY_*` values.
    ///
    /// Rather than using one of the Private Use Areas of Unicode, we use the
    /// area beyond the 17 × 65_536-entry Planes (1_114_112).
    /// We round up to 5_000 so that it's trivial to identify synthesized
    /// characters based on their numeric definition here.
    /// This is safe, since we needn't convert these synthesized characters
    /// into UTF8 (they would otherwise require more than four bytes).
    pub const PRETERUNICODEBASE: u32 = 1_115_000;

    const fn preterunicode(w: u32) -> u32 {
        w + PRETERUNICODEBASE
    }

    pub const NCKEY_INVALID: u32 = preterunicode(0);
    /// we received SIGWINCH
    pub const NCKEY_RESIZE: u32 = preterunicode(1);
    pub const NCKEY_UP: u32 = preterunicode(2);
    pub const NCKEY_RIGHT: u32 = preterunicode(3);
    pub const NCKEY_DOWN: u32 = preterunicode(4);
    pub const NCKEY_LEFT: u32 = preterunicode(5);
    pub const NCKEY_INS: u32 = preterunicode(6);
    pub const NCKEY_DEL: u32 = preterunicode(7);
    pub const NCKEY_BACKSPACE: u32 = preterunicode(8);
    pub const NCKEY_PGDOWN: u32 = preterunicode(9);
    pub const NCKEY_PGUP: u32 = preterunicode(10);
    pub const NCKEY_HOME: u32 = preterunicode(11);
    pub const NCKEY_END: u32 = preterunicode(12);
    pub const NCKEY_F00: u32 = preterunicode(20);
    pub const NCKEY_F01: u32 = preterunicode(21);
    pub const NCKEY_F02: u32 = preterunicode(22);
    pub const NCKEY_F03: u32 = preterunicode(23);
    pub const NCKEY_F04: u32 = preterunicode(24);
    pub const NCKEY_F05: u32 = preterunicode(25);
    pub const NCKEY_F06: u32 = preterunicode(26);
    pub const NCKEY_F07: u32 = preterunicode(27);
    pub const NCKEY_F08: u32 = preterunicode(28);
    pub const NCKEY_F09: u32 = preterunicode(29);
    pub const NCKEY_F10: u32 = preterunicode(30);
    pub const NCKEY_F11: u32 = preterunicode(31);
    pub const NCKEY_F12: u32 = preterunicode(32);
    pub const NCKEY_F13: u32 = preterunicode(33);
    pub const NCKEY_F14: u32 = preterunicode(34);
    pub const NCKEY_F15: u32 = preterunicode(35);
    pub const NCKEY_F16: u32 = preterunicode(36);
    pub const NCKEY_F17: u32 = preterunicode(37);
    pub const NCKEY_F18: u32 = preterunicode(38);
    pub const NCKEY_F19: u32 = preterunicode(39);
    pub const NCKEY_F20: u32 = preterunicode(40);
    pub const NCKEY_F21: u32 = preterunicode(41);
    pub const NCKEY_F22: u32 = preterunicode(42);
    pub const NCKEY_F23: u32 = preterunicode(43);
    pub const NCKEY_F24: u32 = preterunicode(44);
    pub const NCKEY_F25: u32 = preterunicode(45);
    pub const NCKEY_F26: u32 = preterunicode(46);
    pub const NCKEY_F27: u32 = preterunicode(47);
    pub const NCKEY_F28: u32 = preterunicode(48);
    pub const NCKEY_F29: u32 = preterunicode(49);
    pub const NCKEY_F30: u32 = preterunicode(50);
    pub const NCKEY_F31: u32 = preterunicode(51);
    pub const NCKEY_F32: u32 = preterunicode(52);
    pub const NCKEY_F33: u32 = preterunicode(53);
    pub const NCKEY_F34: u32 = preterunicode(54);
    pub const NCKEY_F35: u32 = preterunicode(55);
    pub const NCKEY_F36: u32 = preterunicode(56);
    pub const NCKEY_F37: u32 = preterunicode(57);
    pub const NCKEY_F38: u32 = preterunicode(58);
    pub const NCKEY_F39: u32 = preterunicode(59);
    pub const NCKEY_F40: u32 = preterunicode(60);
    pub const NCKEY_F41: u32 = preterunicode(61);
    pub const NCKEY_F42: u32 = preterunicode(62);
    pub const NCKEY_F43: u32 = preterunicode(63);
    pub const NCKEY_F44: u32 = preterunicode(64);
    pub const NCKEY_F45: u32 = preterunicode(65);
    pub const NCKEY_F46: u32 = preterunicode(66);
    pub const NCKEY_F47: u32 = preterunicode(67);
    pub const NCKEY_F48: u32 = preterunicode(68);
    pub const NCKEY_F49: u32 = preterunicode(69);
    pub const NCKEY_F50: u32 = preterunicode(70);
    pub const NCKEY_F51: u32 = preterunicode(71);
    pub const NCKEY_F52: u32 = preterunicode(72);
    pub const NCKEY_F53: u32 = preterunicode(73);
    pub const NCKEY_F54: u32 = preterunicode(74);
    pub const NCKEY_F55: u32 = preterunicode(75);
    pub const NCKEY_F56: u32 = preterunicode(76);
    pub const NCKEY_F57: u32 = preterunicode(77);
    pub const NCKEY_F58: u32 = preterunicode(78);
    pub const NCKEY_F59: u32 = preterunicode(79);
    pub const NCKEY_F60: u32 = preterunicode(80);

    // ... leave room for function keys.

    pub const NCKEY_ENTER: u32 = preterunicode(121);
    /// "clear-screen or erase"
    pub const NCKEY_CLS: u32 = preterunicode(122);
    /// down + left on keypad
    pub const NCKEY_DLEFT: u32 = preterunicode(123);
    pub const NCKEY_DRIGHT: u32 = preterunicode(124);
    /// up + left on keypad
    pub const NCKEY_ULEFT: u32 = preterunicode(125);
    pub const NCKEY_URIGHT: u32 = preterunicode(126);
    pub const NCKEY_CENTER: u32 = preterunicode(127);
    pub const NCKEY_BEGIN: u32 = preterunicode(128);
    pub const NCKEY_CANCEL: u32 = preterunicode(129);
    pub const NCKEY_CLOSE: u32 = preterunicode(130);
    pub const NCKEY_COMMAND: u32 = preterunicode(131);
    pub const NCKEY_COPY: u32 = preterunicode(132);
    pub const NCKEY_EXIT: u32 = preterunicode(133);
    pub const NCKEY_PRINT: u32 = preterunicode(134);
    pub const NCKEY_REFRESH: u32 = preterunicode(135);

    // these keys aren't generally available outside of the kitty protocol:

    pub const NCKEY_CAPS_LOCK: u32 = preterunicode(150);
    pub const NCKEY_SCROLL_LOCK: u32 = preterunicode(151);
    pub const NCKEY_NUM_LOCK: u32 = preterunicode(152);
    pub const NCKEY_PRINT_SCREEN: u32 = preterunicode(153);
    pub const NCKEY_PAUSE: u32 = preterunicode(154);
    pub const NCKEY_MENU: u32 = preterunicode(155);

    // media keys, similarly only available through kitty's protocol:

    pub const NCKEY_MEDIA_PLAY: u32 = preterunicode(158);
    pub const NCKEY_MEDIA_PAUSE: u32 = preterunicode(159);
    pub const NCKEY_MEDIA_PPAUSE: u32 = preterunicode(160);
    pub const NCKEY_MEDIA_REV: u32 = preterunicode(161);
    pub const NCKEY_MEDIA_STOP: u32 = preterunicode(162);
    pub const NCKEY_MEDIA_FF: u32 = preterunicode(163);
    pub const NCKEY_MEDIA_REWIND: u32 = preterunicode(164);
    pub const NCKEY_MEDIA_NEXT: u32 = preterunicode(165);
    pub const NCKEY_MEDIA_PREV: u32 = preterunicode(166);
    pub const NCKEY_MEDIA_RECORD: u32 = preterunicode(167);
    pub const NCKEY_MEDIA_LVOL: u32 = preterunicode(168);
    pub const NCKEY_MEDIA_RVOL: u32 = preterunicode(169);
    pub const NCKEY_MEDIA_MUTE: u32 = preterunicode(170);

    // modifiers when pressed by themselves. this ordering comes from the Kitty
    // keyboard protocol, and mustn't be changed without updating handlers:

    pub const NCKEY_LSHIFT: u32 = preterunicode(171);
    pub const NCKEY_LCTRL: u32 = preterunicode(172);
    pub const NCKEY_LALT: u32 = preterunicode(173);
    pub const NCKEY_LSUPER: u32 = preterunicode(174);
    pub const NCKEY_LHYPER: u32 = preterunicode(175);
    pub const NCKEY_LMETA: u32 = preterunicode(176);
    pub const NCKEY_RSHIFT: u32 = preterunicode(177);
    pub const NCKEY_RCTRL: u32 = preterunicode(178);
    pub const NCKEY_RALT: u32 = preterunicode(179);
    pub const NCKEY_RSUPER: u32 = preterunicode(180);
    pub const NCKEY_RHYPER: u32 = preterunicode(181);
    pub const NCKEY_RMETA: u32 = preterunicode(182);
    /// `AltGr` in european keyboards
    pub const NCKEY_L3SHIFT: u32 = preterunicode(183);
    pub const NCKEY_L5SHIFT: u32 = preterunicode(184);

    // Mouse events. We encode which button was pressed into the char,
    // but position information is embedded in the larger ncinput event:

    /// no buttons pressed
    pub const NCKEY_MOTION: u32 = preterunicode(200);
    pub const NCKEY_BUTTON1: u32 = preterunicode(201);
    pub const NCKEY_BUTTON2: u32 = preterunicode(202);
    pub const NCKEY_BUTTON3: u32 = preterunicode(203);
    /// scrollwheel up
    pub const NCKEY_BUTTON4: u32 = preterunicode(204);
    /// scrollwheel down
    pub const NCKEY_BUTTON5: u32 = preterunicode(205);
    pub const NCKEY_BUTTON6: u32 = preterunicode(206);
    pub const NCKEY_BUTTON7: u32 = preterunicode(207);
    pub const NCKEY_BUTTON8: u32 = preterunicode(208);
    pub const NCKEY_BUTTON9: u32 = preterunicode(209);
    pub const NCKEY_BUTTON10: u32 = preterunicode(210);
    pub const NCKEY_BUTTON11: u32 = preterunicode(211);

    /// we received SIGCONT
    pub const NCKEY_SIGNAL: u32 = preterunicode(400);

    /// Indicates that we have reached the end of input. Any further calls
    /// will continue to return this immediately.
    pub const NCKEY_EOF: u32 = preterunicode(500);

    // Synonyms (so far as we're concerned):

    /// Alias of [`NCKEY_BUTTON4`].
    pub const NCKEY_SCROLL_UP: u32 = NCKEY_BUTTON4;
    /// Alias of [`NCKEY_BUTTON5`].
    pub const NCKEY_SCROLL_DOWN: u32 = NCKEY_BUTTON5;
    /// Alias of [`NCKEY_ENTER`].
    pub const NCKEY_RETURN: u32 = NCKEY_ENTER;

    // Aliases, from the 128 characters common to ASCII+UTF8:

    pub const NCKEY_TAB: u32 = 0x09;
    pub const NCKEY_ESC: u32 = 0x1b;
    pub const NCKEY_SPACE: u32 = 0x20;
}
