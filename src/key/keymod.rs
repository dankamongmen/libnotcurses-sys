//!

use super::constants;

/// [`NcKey`][crate::NcKey] modifiers bitmask.
#[repr(u32)]
#[derive(Clone, Copy, Debug)]
pub enum NcKeyMod {
    None = 0,
    Shift = constants::NCKEY_MOD_SHIFT,
    Ctrl = constants::NCKEY_MOD_CTRL,
    Alt = constants::NCKEY_MOD_ALT,
    Meta = constants::NCKEY_MOD_META,
}

impl NcKeyMod {
    pub const Control: Self = Self::Ctrl;
}

impl From<NcKeyMod> for u32 {
    fn from(km: NcKeyMod) -> Self {
        use NcKeyMod::*;
        match km {
            NcKeyMod::None => 0,
            Shift => constants::NCKEY_MOD_SHIFT,
            Ctrl => constants::NCKEY_MOD_CTRL,
            Alt => constants::NCKEY_MOD_ALT,
            Meta => constants::NCKEY_MOD_META,
        }
    }
}

impl From<u32> for NcKeyMod {
    fn from(value: u32) -> Self {
        use NcKeyMod::*;
        match value {
            constants::NCKEY_MOD_SHIFT => Shift,
            constants::NCKEY_MOD_CTRL => Ctrl,
            constants::NCKEY_MOD_ALT => Alt,
            constants::NCKEY_MOD_META => Meta,
            _ => None,
        }
    }
}
