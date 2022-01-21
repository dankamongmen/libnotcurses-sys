//! NcAlpha

#[allow(unused_imports)] // for doc comments
use crate::NcCell;

/// Alpha information, part of an [`NcChannel`][crate::NcChannel],
/// applies to [`NcCell`]'s foreground or background color.
///
/// # Default:
/// *[`NcAlpha::Opaque`]*
///
/// ## Diagram
///
/// Internally it's 2 bits of alpha, surrounded by context dependent bits:
///
/// ```txt
/// ~~AA~~~~ -------- -------- --------
/// ```
///
/// See also: [`NcChannels`][crate::NcChannels] for more context information.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NcAlpha {
    /// Indicates [`NcCell`]'s foreground or background color will be a
    /// composite between its color and the `NcCell`s' corresponding colors
    /// underneath it.
    Blend = c_api::NCALPHA_BLEND,

    /// Indicates the foreground color will be high-contrast,
    /// relative to the computed background.
    ///
    /// Background cannot be high-contrast.
    HighContrast = c_api::NCALPHA_HIGHCONTRAST,

    /// Indicates [`NcCell`]'s foreground or background color is used unchanged.
    Opaque = c_api::NCALPHA_OPAQUE,

    /// Indicates [`NcCell`]'s foreground or background color is derived
    /// entirely from the `NcCell`s underneath it.
    Transparent = c_api::NCALPHA_TRANSPARENT,
}

mod std_impls {
    use super::{c_api, NcAlpha};
    use std::fmt;

    impl Default for NcAlpha {
        fn default() -> Self {
            Self::Opaque
        }
    }

    impl fmt::Display for NcAlpha {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            use NcAlpha::*;
            write!(
                f,
                "{}",
                match self {
                    Blend => "Blend",
                    HighContrast => "HighContrast",
                    Opaque => "Opaque",
                    Transparent => "Transparent",
                }
            )
        }
    }

    impl From<c_api::NcAlpha_u32> for NcAlpha {
        fn from(alpha: c_api::NcAlpha_u32) -> Self {
            use {c_api::*, NcAlpha::*};
            match alpha {
                NCALPHA_BLEND => Blend,
                NCALPHA_HIGHCONTRAST => HighContrast,
                NCALPHA_OPAQUE => Opaque,
                NCALPHA_TRANSPARENT => Transparent,
                _ => Self::default(),
            }
        }
    }

    impl From<NcAlpha> for c_api::NcAlpha_u32 {
        fn from(alpha: NcAlpha) -> Self {
            use {c_api::*, NcAlpha::*};
            match alpha {
                Blend => NCALPHA_BLEND,
                HighContrast => NCALPHA_HIGHCONTRAST,
                Opaque => NCALPHA_OPAQUE,
                Transparent => NCALPHA_TRANSPARENT,
            }
        }
    }
}

pub(crate) mod c_api {
    use crate::c_api::ffi;

    #[allow(unused_imports)] // for doc comments
    use crate::NcCell;

    /// 2 bits of alpha (surrounded by context dependent bits)
    /// part of an [`NcChannel`][crate::NcChannel].
    ///
    /// It's recommended to use [`NcAlpha`][crate::NcAlpha] instead.
    ///
    /// # Associated `c_api` constants
    ///
    /// - [`NCALPHA_BLEND`]
    /// - [`NCALPHA_HIGHCONTRAST`]
    /// - [`NCALPHA_OPAQUE`]
    /// - [`NCALPHA_TRANSPARENT`]
    ///
    /// ## Diagram
    ///
    /// ```txt
    /// ~~AA~~~~ -------- -------- --------
    /// ```
    /// `type in C: no data type`
    pub type NcAlpha_u32 = u32;

    /// [`NcAlpha_u32`] bits indicating [`NcCell`]'s foreground or background color
    /// will be a composite between its color and the `NcCell`s' corresponding
    /// colors underneath it.
    pub const NCALPHA_BLEND: NcAlpha_u32 = ffi::NCALPHA_BLEND;

    /// [`NcAlpha_u32`] bits indicating [`NcCell`]'s foreground color will be
    /// high-contrast (relative to the computed background).
    /// Background cannot be high-contrast.
    pub const NCALPHA_HIGHCONTRAST: NcAlpha_u32 = ffi::NCALPHA_HIGHCONTRAST;

    /// [`NcAlpha_u32`] bits indicating [`NcCell`]'s foreground or background color
    /// is used unchanged.
    pub const NCALPHA_OPAQUE: NcAlpha_u32 = ffi::NCALPHA_OPAQUE;

    /// [`NcAlpha_u32`] bits indicating [`NcCell`]'s foreground or background color
    /// is derived entirely from the `NcCell`s underneath it.
    pub const NCALPHA_TRANSPARENT: NcAlpha_u32 = ffi::NCALPHA_TRANSPARENT;
}
