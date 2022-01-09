//! `NcKey`

// functions manually reimplemented: 2
// ------------------------------------------
// (+) done: 2
// (#) test: 0
// ------------------------------------------
// + nckey_mouse_p
// + nckey_synthesized_p

pub(crate) mod reimplemented;

#[allow(clippy::module_inception)]
mod key;
mod keymod;

pub use {key::NcKey, keymod::NcKeyMod};

pub(crate) mod constants {
    use crate::c_api::ffi;

    /// Rather than using one of the Private Use Areas of Unicode, we use the
    /// area beyond the 17 65536-entry Planes (1114112).
    ///
    /// We round up to 5000 so that it's trivial to identify synthesized
    /// characters based on their numeric definition here.
    ///
    /// This is safe, since we needn't convert these synthesized characters
    /// into UTF8 (they would otherwise require more than four bytes).
    pub const PRETERUNICODEBASE: u32 = 1115000;

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
    /// backspace (sometimes)
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
    /// the most truly neutral of keypresses
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
    /// will continute to return this immediately.
    pub const NCKEY_EOF: u32 = preterunicode(500);

    // Synonyms (so far as we're concerned):

    pub const NCKEY_SCROLL_UP: u32 = NCKEY_BUTTON4;
    pub const NCKEY_SCROLL_DOWN: u32 = NCKEY_BUTTON5;
    pub const NCKEY_RETURN: u32 = NCKEY_ENTER;

    // Aliases, from the 128 characters common to ASCII+UTF8:

    pub const NCKEY_TAB: u32 = 0x09;
    pub const NCKEY_ESC: u32 = 0x1b;
    pub const NCKEY_SPACE: u32 = 0x20;

    // Modifiers bitmask

    pub const NCKEY_MOD_SHIFT: u32 = ffi::NCKEY_MOD_SHIFT;
    pub const NCKEY_MOD_CTRL: u32 = ffi::NCKEY_MOD_CTRL;
    pub const NCKEY_MOD_ALT: u32 = ffi::NCKEY_MOD_ALT;
    pub const NCKEY_MOD_META: u32 = ffi::NCKEY_MOD_META;
}
