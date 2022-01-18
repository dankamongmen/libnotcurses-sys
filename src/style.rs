//!

/// A bitmask of styles.
///
/// # Flags
/// - [`Bold`][NcStyle::Bold]
/// - [`Italic`][NcStyle::Italic]
/// - [`Struck`][NcStyle::Struck]
/// - [`Underline`][NcStyle::Underline]
/// - [`Undercurl`][NcStyle::Undercurl]
/// - [`None`][NcStyle::None]
/// - [`Mask`][NcStyle::Mask]
///
/// # Default
/// *[`NcStyle::None`]
///
/// # Notes
/// - if you want reverse video, try [`NcChannels.reverse`]
/// - if you want blink, try [`NcPlane.pulse`].
/// - if you want protection, put things on a different `NcPlane`.
///
/// [`NcChannels.reverse`]: crate::NcChannels#method.reverse
/// [`NcPlane.pulse`]: crate::NcPlane#method.pulse

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NcStyle(pub c_api::NcStyle_u16);

/// # Constants
impl NcStyle {
    /// Bold.
    pub const Bold: Self = Self(c_api::NCSTYLE_BOLD);

    /// Italic.
    pub const Italic: Self = Self(c_api::NCSTYLE_ITALIC);

    /// Struck.
    pub const Struck: Self = Self(c_api::NCSTYLE_STRUCK);

    /// Underline.
    pub const Underline: Self = Self(c_api::NCSTYLE_UNDERLINE);

    /// Undercurl.
    pub const Undercurl: Self = Self(c_api::NCSTYLE_UNDERCURL);

    /// None of the styles (all bits set to 0).
    pub const None: Self = Self(0);

    /// The mask of all styles (all bits set to 1).
    pub const Mask: Self = Self(c_api::NCSTYLE_MASK);
}

mod std_impls {
    use super::{c_api::NcStyle_u16, NcStyle};
    use std::fmt;

    impl Default for NcStyle {
        fn default() -> Self {
            Self::None
        }
    }

    crate::unit_impl_from![NcStyle, NcStyle_u16];

    // for ncplane_*_styles & ncdirect_*_styles:
    impl From<NcStyle> for u32 {
        fn from(style: NcStyle) -> Self {
            style.0 as u32
        }
    }

    crate::unit_impl_ops![bitwise; NcStyle];

    impl fmt::UpperHex for NcStyle {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let val = self.0;
            fmt::UpperHex::fmt(&val, f)
        }
    }
}

/// # Methods
impl NcStyle {
    crate::from_primitive![c_api::NcStyle_u16];

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
