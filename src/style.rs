//!

use std::fmt;

/// Style bitmask.
///
/// # Default
/// *[`NcStyle::None`]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NcStyle(c_api::NcStyle_u16);

/// # Constants
impl NcStyle {
    /// None of the styles (all bits set to 0).
    pub const None: Self = Self(0);

    /// The style mask (all bits set to 1).
    pub const Mask: Self = Self(c_api::NCSTYLE_MASK);

    ///
    pub const Italic: Self = Self(c_api::NCSTYLE_ITALIC);

    ///
    pub const Underline: Self = Self(c_api::NCSTYLE_UNDERLINE);

    ///
    pub const Undercurl: Self = Self(c_api::NCSTYLE_UNDERCURL);

    ///
    pub const Struck: Self = Self(c_api::NCSTYLE_STRUCK);

    ///
    pub const Bold: Self = Self(c_api::NCSTYLE_BOLD);
}

impl Default for NcStyle {
    fn default() -> Self {
        Self::None
    }
}

// from (WIP) TODO: macro
// - all variants

impl From<NcStyle> for c_api::NcStyle_u16 {
    fn from(style: NcStyle) -> Self {
        style.0
    }
}
impl<'a> From<&'a mut NcStyle> for &'a mut c_api::NcStyle_u16 {
    fn from(style: &'a mut NcStyle) -> Self {
        &mut style.0
    }
}
// TEMP?
impl From<&mut NcStyle> for *mut c_api::NcStyle_u16 {
    fn from(style: &mut NcStyle) -> Self {
        &mut style.0 as *mut c_api::NcStyle_u16
    }
}

impl From<c_api::NcStyle_u16> for NcStyle {
    fn from(value: c_api::NcStyle_u16) -> Self {
        Self(value)
    }
}

impl From<NcStyle> for u32 {
    fn from(style: NcStyle) -> Self {
        style.0 as u32
    }
}

impl fmt::UpperHex for NcStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = self.0;
        fmt::UpperHex::fmt(&val, f)
    }
}

impl NcStyle {
    /// Returns a `Vec` with all the styles contained in the current style.
    pub fn to_vec(&self) -> Vec<NcStyle> {
        let mut v = vec![];
        let styles = [
            NcStyle::Italic,
            NcStyle::Underline,
            NcStyle::Undercurl,
            NcStyle::Struck,
            NcStyle::Bold,
            NcStyle::None,
        ];
        for s in &styles {
            if self.has(*s) {
                v.push(*s)
            }
        }
        v
    }

    /// Returns true if the current style has included the `other_style`.
    pub fn has(&self, other: NcStyle) -> bool {
        (self.0 & other.0) == other.0
    }

    /// Adds the `other_style` to the current style.
    pub fn add(&mut self, other: NcStyle) {
        self.0 |= other.0
    }
}

pub(crate) mod c_api {
    use crate::bindings::ffi;

    /// Styling attribute flags.
    ///
    /// It's recommended to use [`NcStyle`][crate::NcStyle] instead.
    ///
    /// `type in C:  uint16_t`
    ///
    /// # Associated `c_api` constants
    ///
    /// - [`NCSTYLE_ITALIC`]
    /// - [`NCSTYLE_UNDERLINE`]
    /// - [`NCSTYLE_UNDERCURL`]
    /// - [`NCSTYLE_STRUCK`]
    /// - [`NCSTYLE_BOLD`]
    /// - [`NCSTYLE_NONE`]
    /// - [`NCSTYLE_MASK`]
    ///
    pub type NcStyle_u16 = u16;

    /// [`NcStyle_u16`] Italic.
    pub const NCSTYLE_ITALIC: u16 = ffi::NCSTYLE_ITALIC as u16;

    /// [`NcStyle_u16`] Underline.
    pub const NCSTYLE_UNDERLINE: u16 = ffi::NCSTYLE_UNDERLINE as u16;

    /// [`NcStyle_u16`] Undercurl.
    pub const NCSTYLE_UNDERCURL: u16 = ffi::NCSTYLE_UNDERCURL as u16;

    /// [`NcStyle_u16`] Struck.
    pub const NCSTYLE_STRUCK: u16 = ffi::NCSTYLE_STRUCK as u16;

    /// [`NcStyle_u16`] Bold.
    pub const NCSTYLE_BOLD: u16 = ffi::NCSTYLE_BOLD as u16;

    /// [`NcStyle_u16`] None.
    pub const NCSTYLE_NONE: u16 = ffi::NCSTYLE_NONE as u16;

    /// [`NcStyle_u16`] mask.
    pub const NCSTYLE_MASK: u16 = ffi::NCSTYLE_MASK as u16;
}
