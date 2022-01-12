//!

use super::constants;

/// [`NcKey`][crate::NcKey] modifiers bitmask.
#[repr(u32)]
#[derive(Clone, Copy, Debug)]
pub enum NcKeyMod {
    None = 0,
    Shift = constants::NCKEY_MOD_SHIFT,
    Alt = constants::NCKEY_MOD_ALT,
    Ctrl = constants::NCKEY_MOD_CTRL,
    Super = constants::NCKEY_MOD_SUPER,
    Hyper = constants::NCKEY_MOD_HYPER,
    Meta = constants::NCKEY_MOD_META,
    CapsLock = constants::NCKEY_MOD_CAPSLOCK,
    NumLock = constants::NCKEY_MOD_NUMLOCK,
}

/// # Aliases
impl NcKeyMod {
    pub const Control: Self = Self::Ctrl;
}

impl Default for NcKeyMod {
    fn default() -> Self {
        Self::None
    }
}

impl From<NcKeyMod> for u32 {
    fn from(km: NcKeyMod) -> Self {
        use NcKeyMod::*;
        match km {
            NcKeyMod::None => 0,
            Shift => constants::NCKEY_MOD_SHIFT,
            Alt => constants::NCKEY_MOD_ALT,
            Ctrl => constants::NCKEY_MOD_CTRL,
            Super => constants::NCKEY_MOD_SUPER,
            Hyper => constants::NCKEY_MOD_HYPER,
            Meta => constants::NCKEY_MOD_META,
            CapsLock => constants::NCKEY_MOD_CAPSLOCK,
            NumLock => constants::NCKEY_MOD_NUMLOCK,
        }
    }
}

impl From<u32> for NcKeyMod {
    fn from(value: u32) -> Self {
        use NcKeyMod::*;
        match value {
            constants::NCKEY_MOD_SHIFT => Shift,
            constants::NCKEY_MOD_ALT => Alt,
            constants::NCKEY_MOD_CTRL => Ctrl,
            constants::NCKEY_MOD_SUPER => Super,
            constants::NCKEY_MOD_HYPER => Hyper,
            constants::NCKEY_MOD_META => Meta,
            constants::NCKEY_MOD_CAPSLOCK => CapsLock,
            constants::NCKEY_MOD_NUMLOCK => NumLock,
            _ => None,
        }
    }
}
