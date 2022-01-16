//!

use super::constants;
use core::ops::{BitAnd, BitOr, Not};

/// [`NcKey`][crate::NcKey] modifiers bitmask.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NcKeyMod(u32);

/// # Constants
impl NcKeyMod {
    pub const None: Self = Self(0);
    pub const Shift: Self = Self(constants::NCKEY_MOD_SHIFT);
    pub const Alt: Self = Self(constants::NCKEY_MOD_ALT);
    pub const Ctrl: Self = Self(constants::NCKEY_MOD_CTRL);
    pub const Super: Self = Self(constants::NCKEY_MOD_SUPER);
    pub const Hyper: Self = Self(constants::NCKEY_MOD_HYPER);
    pub const Meta: Self = Self(constants::NCKEY_MOD_META);
    pub const CapsLock: Self = Self(constants::NCKEY_MOD_CAPSLOCK);
    pub const NumLock: Self = Self(constants::NCKEY_MOD_NUMLOCK);
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

// WIP
// impl PartialEq<NcKeyMod> for u32 {
//     fn eq(&self, other: &NcKeyMod) -> bool {
//         self == &other.0
//     }
// }

impl Not for NcKeyMod {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(self.0.not())
    }
}

impl BitOr for NcKeyMod {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}
impl BitOr<u32> for NcKeyMod {
    type Output = Self;
    fn bitor(self, rhs: u32) -> Self::Output {
        Self(self.0 | rhs)
    }
}
impl BitOr<NcKeyMod> for u32 {
    type Output = NcKeyMod;
    fn bitor(self, rhs: Self::Output) -> Self::Output {
        NcKeyMod(self | rhs.0)
    }
}

impl BitAnd for NcKeyMod {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}
impl BitAnd<u32> for NcKeyMod {
    type Output = Self;
    fn bitand(self, rhs: u32) -> Self::Output {
        Self(self.0 & rhs)
    }
}
impl BitAnd<NcKeyMod> for u32 {
    type Output = NcKeyMod;
    fn bitand(self, rhs: Self::Output) -> Self::Output {
        NcKeyMod(self & rhs.0)
    }
}

impl From<NcKeyMod> for u32 {
    fn from(keymod: NcKeyMod) -> Self {
        keymod.0
    }
}
impl From<u32> for NcKeyMod {
    fn from(value: u32) -> Self {
        match value {
            constants::NCKEY_MOD_SHIFT => Self::Shift,
            constants::NCKEY_MOD_ALT => Self::Alt,
            constants::NCKEY_MOD_CTRL => Self::Ctrl,
            constants::NCKEY_MOD_SUPER => Self::Super,
            constants::NCKEY_MOD_HYPER => Self::Hyper,
            constants::NCKEY_MOD_META => Self::Meta,
            constants::NCKEY_MOD_CAPSLOCK => Self::CapsLock,
            constants::NCKEY_MOD_NUMLOCK => Self::NumLock,
            _ => Self::None,
        }
    }
}
