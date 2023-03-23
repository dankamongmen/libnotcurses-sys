//!

#[cfg(not(feature = "std"))]
use alloc::{vec, vec::Vec};

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
/// - if you want protection, put things on a different [`NcPlane`].
///
/// [`NcChannels.reverse`]: crate::NcChannels#method.reverse
/// [`NcPlane.pulse`]: crate::NcPlane#method.pulse
/// [`NcPlane`]: crate::NcPlane
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct NcStyle(pub c_api::NcStyle_u16);

/// # Flags
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
    use core::fmt;

    #[cfg(not(feature = "std"))]
    use alloc::string::String;

    use super::{c_api::NcStyle_u16, NcStyle};

    impl Default for NcStyle {
        fn default() -> Self {
            Self::None
        }
    }

    impl fmt::Display for NcStyle {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let mut string = String::new();
            for s in self.to_vec() {
                string.push_str(match s {
                    NcStyle::Italic => "Italic ",
                    NcStyle::Underline => "Underline ",
                    NcStyle::Undercurl => "Undercurl ",
                    NcStyle::Struck => "Struck ",
                    NcStyle::Bold => "Bold ",
                    NcStyle::None => "None ",
                    _ => "",
                });
            }
            let _ = string.pop();
            write!(f, "{}", string)
        }
    }
    impl fmt::Debug for NcStyle {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let mut string = String::new();
            for s in self.to_vec() {
                string.push_str(match s {
                    NcStyle::Italic => "Italic+",
                    NcStyle::Underline => "Underline+",
                    NcStyle::Undercurl => "Undercurl+",
                    NcStyle::Struck => "Struck+",
                    NcStyle::Bold => "Bold+",
                    NcStyle::None => "None ",
                    _ => "",
                });
            }
            let _ = string.pop();
            write!(f, "NcStyle::{}", string)
        }
    }

    crate::from_primitive![NcStyle, NcStyle_u16];

    crate::unit_impl_from![NcStyle, NcStyle_u16];

    // for ncplane_*_styles & ncdirect_*_styles:
    impl From<NcStyle> for u32 {
        fn from(style: NcStyle) -> Self {
            style.0 as u32
        }
    }

    crate::unit_impl_ops![bitwise; NcStyle, NcStyle_u16];
    crate::unit_impl_fmt![bases; NcStyle];
}

/// # Methods
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
        ];
        for s in &styles {
            if self.has(*s) {
                v.push(*s)
            }
        }
        // only push None if there are no other styles present.
        if v.is_empty() {
            v.push(NcStyle::None)
        }
        v
    }

    /// Returns true if the current style has included the `other_style`.
    #[inline]
    pub fn has(&self, other: impl Into<NcStyle>) -> bool {
        let other = other.into();
        (self.0 & other.0) == other.0
    }

    /// Sets the `other` style in the current style.
    #[inline]
    pub fn set(&mut self, other: impl Into<NcStyle>) {
        self.0 |= other.into().0
    }

    /// Unsets the `other` style in the current style.
    #[inline]
    pub fn unset(&mut self, other: impl Into<NcStyle>) {
        self.0 &= !other.into().0
    }
}

pub(crate) mod c_api {
    use crate::c_api::ffi;

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
