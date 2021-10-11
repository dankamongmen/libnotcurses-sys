#[allow(unused_imports)]
use crate::NcCell;

/// 2 bits of alpha (surrounded by context dependent bits).
/// It is part of an [`NcChannel`][crate::NcChannel].
///
/// ## Diagram
///
/// ```txt
/// ~~AA~~~~ -------- -------- --------
/// ```
/// `type in C: no data type`
///
pub type NcAlpha = u32;

crate::impl_api![
    NcAlpha,
    NcAlphaApi,
    /// [`NcAlpha`] bits indicating [`NcCell`]'s foreground or background color
    /// will be a composite between its color and the `NcCell`s' corresponding
    /// colors underneath it.
    const BLEND: u32 = constants::NCALPHA_BLEND;,
    /// [`NcAlpha`] bits indicating [`NcCell`]'s foreground color will be
    /// high-contrast (relative to the computed background).
    /// Background cannot be high-contrast.
    const HIGHCONTRAST: u32 = constants::NCALPHA_HIGHCONTRAST;,
    /// [`NcAlpha`] bits indicating [`NcCell`]'s foreground or background color
    /// is used unchanged.
    const OPAQUE: u32 = constants::NCALPHA_OPAQUE;,
    /// [`NcAlpha`] bits indicating [`NcCell`]'s foreground or background color
    /// is derived entirely from the `NcCell`s underneath it.
    const TRANSPARENT: u32 = constants::NCALPHA_TRANSPARENT;
];

/// Enable the [`NcAlpha`] associated methods and constants.
pub(crate) mod constants {
    #[allow(unused_imports)]
    use crate::{NcAlpha, NcCell};

    /// [`NcAlpha`] bits indicating [`NcCell`]'s foreground or background color
    /// will be a composite between its color and the `NcCell`s' corresponding
    /// colors underneath it.
    pub const NCALPHA_BLEND: NcAlpha = crate::bindings::ffi::NCALPHA_BLEND;

    /// [`NcAlpha`] bits indicating [`NcCell`]'s foreground color will be
    /// high-contrast (relative to the computed background).
    /// Background cannot be high-contrast.
    pub const NCALPHA_HIGHCONTRAST: NcAlpha = crate::bindings::ffi::NCALPHA_HIGHCONTRAST;

    /// [`NcAlpha`] bits indicating [`NcCell`]'s foreground or background color
    /// is used unchanged.
    pub const NCALPHA_OPAQUE: NcAlpha = crate::bindings::ffi::NCALPHA_OPAQUE;

    /// [`NcAlpha`] bits indicating [`NcCell`]'s foreground or background color
    /// is derived entirely from the `NcCell`s underneath it.
    pub const NCALPHA_TRANSPARENT: NcAlpha = crate::bindings::ffi::NCALPHA_TRANSPARENT;
}
