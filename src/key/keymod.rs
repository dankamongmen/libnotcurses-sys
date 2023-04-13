//!

/// A bitmask of [`NcKey`][crate::NcKey] modifiers.
///
/// # Default
/// *[`NcKeyMod::None`]
///
/// # Flags
/// - [`Shift`][NcKeyMod::Shift]
/// - [`Alt`][NcKeyMod::Alt]
/// - [`Ctrl`][NcKeyMod::Ctrl]
/// - [`Super`][NcKeyMod::Super]
/// - [`Hyper`][NcKeyMod::Hyper]
/// - [`Meta`][NcKeyMod::Meta]
/// - [`CapsLock`][NcKeyMod::CapsLock]
/// - [`NumLock`][NcKeyMod::NumLock]
/// - [`None`][NcKeyMod::None]
/// - [`Mask`][NcKeyMod::Mask]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct NcKeyMod(pub u32);

/// # Flags
impl NcKeyMod {
    ///
    pub const Shift: Self = Self(c_api::NCKEY_MOD_SHIFT);

    ///
    pub const Alt: Self = Self(c_api::NCKEY_MOD_ALT);

    ///
    pub const Ctrl: Self = Self(c_api::NCKEY_MOD_CTRL);

    ///
    pub const Super: Self = Self(c_api::NCKEY_MOD_SUPER);

    ///
    pub const Hyper: Self = Self(c_api::NCKEY_MOD_HYPER);

    ///
    pub const Meta: Self = Self(c_api::NCKEY_MOD_META);

    ///
    pub const CapsLock: Self = Self(c_api::NCKEY_MOD_CAPSLOCK);

    ///
    pub const NumLock: Self = Self(c_api::NCKEY_MOD_NUMLOCK);

    /// None of the modifiers (all bits set to 0).
    pub const None: Self = Self(0);

    /// The modifier mask (all bits set to 1).
    pub const Mask: Self = Self(u32::MAX);
}

/// # Aliases
impl NcKeyMod {
    ///
    pub const Control: Self = Self::Ctrl;
}

/// # Methods
impl NcKeyMod {
    /// Returns true if no modifiers are present.
    pub fn none_p(&self) -> bool {
        *self == NcKeyMod::None
    }

    /// Returns true if the `Shift` modifier is present.
    pub fn shift_p(&self) -> bool {
        *self & NcKeyMod::Shift != NcKeyMod::None
    }

    /// Returns true if the `Alt` modifier is present.
    pub fn alt_p(&self) -> bool {
        *self & NcKeyMod::Alt != NcKeyMod::None
    }

    /// Returns true if the `Ctrl` modifier is present.
    pub fn ctrl_p(&self) -> bool {
        *self & NcKeyMod::Ctrl != NcKeyMod::None
    }

    /// Returns true if the `Super` modifier is present.
    pub fn super_p(&self) -> bool {
        *self & NcKeyMod::Super != NcKeyMod::None
    }

    /// Returns true if the `Hyper` modifier is present.
    pub fn hyper_p(&self) -> bool {
        *self & NcKeyMod::Hyper != NcKeyMod::None
    }

    /// Returns true if the `Meta` modifier is present.
    pub fn meta_p(&self) -> bool {
        *self & NcKeyMod::Meta != NcKeyMod::None
    }

    /// Returns true if the `CapsLock` modifier is present.
    pub fn capslock_p(&self) -> bool {
        *self & NcKeyMod::CapsLock != NcKeyMod::None
    }

    /// Returns true if the `NumLock` modifier is present.
    pub fn numlock_p(&self) -> bool {
        *self & NcKeyMod::NumLock != NcKeyMod::None
    }
}

mod core_impls {
    use core::fmt;

    #[cfg(not(feature = "std"))]
    use alloc::string::String;

    use super::NcKeyMod;

    impl Default for NcKeyMod {
        fn default() -> Self {
            Self::None
        }
    }

    impl fmt::Display for NcKeyMod {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let mut string = String::new();

            if self.none_p() {
                string += "None ";
            } else {
                if self.capslock_p() {
                    string += "CapsLock+";
                }
                if self.numlock_p() {
                    string += "NumLock+";
                }
                if self.ctrl_p() {
                    string += "Ctrl+";
                }
                if self.shift_p() {
                    string += "Shift+";
                }
                if self.alt_p() {
                    string += "Alt+";
                }
                if self.meta_p() {
                    string += "Meta+";
                }
                if self.super_p() {
                    string += "Super+";
                }
                if self.hyper_p() {
                    string += "Hyper+";
                }
            }
            string.pop();

            write!(f, "{}", string)
        }
    }

    impl fmt::Debug for NcKeyMod {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "NcKeyMod::{}", self)
        }
    }

    crate::from_primitive![NcKeyMod, u32];
    crate::unit_impl_from![NcKeyMod, u32];

    crate::unit_impl_ops![bitwise; NcKeyMod, u32];
}

pub(crate) mod c_api {
    use crate::c_api::ffi;

    ///
    pub const NCKEY_MOD_SHIFT: u32 = ffi::NCKEY_MOD_SHIFT;

    ///
    pub const NCKEY_MOD_ALT: u32 = ffi::NCKEY_MOD_ALT;

    ///
    pub const NCKEY_MOD_CTRL: u32 = ffi::NCKEY_MOD_CTRL;

    ///
    pub const NCKEY_MOD_SUPER: u32 = ffi::NCKEY_MOD_SUPER;

    ///
    pub const NCKEY_MOD_HYPER: u32 = ffi::NCKEY_MOD_HYPER;

    ///
    pub const NCKEY_MOD_META: u32 = ffi::NCKEY_MOD_META;

    ///
    pub const NCKEY_MOD_CAPSLOCK: u32 = ffi::NCKEY_MOD_CAPSLOCK;

    ///
    pub const NCKEY_MOD_NUMLOCK: u32 = ffi::NCKEY_MOD_NUMLOCK;
}
